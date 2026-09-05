//! Bounded, deterministic selection of opaque acquisition references.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const MAX_IDENTIFIER_BYTES: usize = 256;
const MAX_CONCEPT_INPUTS: usize = 128;
const MAX_LIMIT: usize = 4096;

/// A syntactically validated acquisition reference and its declared concept hints.
///
/// A hint is planning input, not evidence that a page contains the concept.
/// Identifiers remain byte-exact opaque keys: this type does not parse URLs,
/// normalize Unicode, authorize origins, resolve credentials, or perform I/O.
/// Debug output reports sizes only; explicit accessors expose the caller's data.
#[derive(Clone, PartialEq, Eq)]
pub struct Candidate {
    id: String,
    concepts: Vec<String>,
}

impl Candidate {
    /// Construct a candidate, retaining distinct hints in first-occurrence order.
    ///
    /// Identifiers must contain 1..=256 UTF-8 bytes, without Unicode whitespace
    /// or control characters. At most 128 raw concept entries are allowed,
    /// before deduplication. Empty hints denote a discovery-only candidate.
    ///
    /// # Errors
    ///
    /// Returns [`FrontierError::InvalidIdentifier`] for an invalid identifier,
    /// or [`FrontierError::InvalidConceptSet`] for excessive raw concept input.
    pub fn new(id: &str, concepts: &[&str]) -> Result<Self, FrontierError> {
        validate_identifier(id)?;
        let concepts = validate_concepts(concepts, false)?;
        Ok(Self {
            id: id.to_owned(),
            concepts,
        })
    }

    /// Return the exact opaque acquisition reference, not an authorization grant.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return distinct concept hints in their original first-occurrence order.
    pub fn concepts(&self) -> &[String] {
        &self.concepts
    }
}

impl fmt::Debug for Candidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Candidate")
            .field("id_bytes", &self.id.len())
            .field("concept_count", &self.concepts.len())
            .finish()
    }
}

/// Per-instance lifetime limits, validated and copied at frontier construction.
///
/// Both values must be in 1..=4096. Draining the pending queue does not reset
/// either limit. The caller owns any wider task, wall-clock, or I/O budgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontierLimits {
    /// Maximum distinct target identities ever admitted by this frontier.
    pub max_candidates: usize,
    /// Maximum candidate dispatches, not a count of actual network operations.
    pub max_attempts: usize,
}

/// A non-reflecting validation or admission failure.
///
/// Errors retain no caller-supplied identifier, concept, URL, or credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierError {
    /// An identifier is empty, oversized, or contains whitespace or controls.
    InvalidIdentifier,
    /// The task has no concepts, or raw concept input exceeds 128 entries.
    InvalidConceptSet,
    /// A candidate or attempt limit is outside 1..=4096.
    InvalidLimits,
    /// At least one candidate hint is absent from the declared task concepts.
    UnknownConcept,
    /// A new identity would exceed the lifetime unique-target limit.
    CandidateLimitReached,
}

impl fmt::Display for FrontierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidIdentifier => "invalid frontier identifier",
            Self::InvalidConceptSet => "invalid frontier concept set",
            Self::InvalidLimits => "invalid frontier limits",
            Self::UnknownConcept => "candidate has an undeclared concept",
            Self::CandidateLimitReached => "frontier lifetime candidate limit reached",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for FrontierError {}

/// One planning outcome; none of these variants certifies successful collection.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "candidate dispatch is planning output, not a collection-success receipt"]
pub enum FrontierStep {
    /// A candidate was removed from the pending queue and one attempt spent.
    Candidate(Candidate),
    /// No candidate is pending; independently validate goal completion elsewhere.
    Drained,
    /// Work remains, but dispatch would exceed the lifetime attempt budget.
    BudgetExhausted,
}

/// A deterministic, bounded semantic crawl frontier without network authority.
///
/// More distinct declared concept hints sort first, then first-admission order.
/// The first admitted candidate for an exact identity wins, even after dispatch.
/// Unknown hints fail before duplicate and capacity checks; all returned errors
/// leave state unchanged. The frontier deliberately exposes no reset or clone.
///
/// A `BTreeMap` stores immutable priority/order keys; a separate lifetime set
/// retains dispatched identities. Ordering uses the lifetime admission count,
/// never pending length, so dispatch/enqueue interleaving cannot overwrite work.
/// This is explicit-hint scheduling, not ontology induction or learned ranking.
/// Acquisition adapters still own URL normalization, authorization and I/O.
pub struct CrawlFrontier {
    required_concepts: BTreeSet<String>,
    pending: BTreeMap<(Reverse<usize>, usize), Candidate>,
    seen: BTreeSet<String>,
    limits: FrontierLimits,
    attempts: usize,
}

impl CrawlFrontier {
    /// Construct an empty frontier with immutable task concepts and limits.
    ///
    /// Required concepts have the same identifier rules as [`Candidate::new`].
    /// Their raw input count must be 1..=128; duplicate entries do not add weight.
    ///
    /// # Errors
    ///
    /// Returns [`FrontierError::InvalidLimits`] for an invalid limit,
    /// [`FrontierError::InvalidConceptSet`] for empty or excessive raw input,
    /// or [`FrontierError::InvalidIdentifier`] for a malformed concept identifier.
    pub fn new(
        required_concepts: &[&str],
        limits: FrontierLimits,
    ) -> Result<Self, FrontierError> {
        if !(1..=MAX_LIMIT).contains(&limits.max_candidates)
            || !(1..=MAX_LIMIT).contains(&limits.max_attempts)
        {
            return Err(FrontierError::InvalidLimits);
        }
        let required_concepts = validate_concepts(required_concepts, true)?;
        Ok(Self {
            required_concepts: required_concepts.into_iter().collect(),
            pending: BTreeMap::new(),
            seen: BTreeSet::new(),
            limits,
            attempts: 0,
        })
    }

    /// Admit a new candidate, or return `false` for a valid duplicate identity.
    ///
    /// Duplicate admission never replaces, reprioritizes, or requeues the first
    /// accepted candidate. Capacity counts all accepted identities, including
    /// dispatched ones. Admission spends no attempt, even after the dispatch
    /// budget is exhausted; such pending work remains visibly undispatched.
    ///
    /// # Errors
    ///
    /// Returns [`FrontierError::UnknownConcept`] before checking identity or
    /// capacity if any hint is undeclared. Otherwise a new identity at capacity
    /// returns [`FrontierError::CandidateLimitReached`]. Neither error mutates
    /// the pending queue, seen ledger, or attempt count.
    pub fn enqueue(&mut self, candidate: Candidate) -> Result<bool, FrontierError> {
        if candidate
            .concepts
            .iter()
            .any(|concept| !self.required_concepts.contains(concept))
        {
            return Err(FrontierError::UnknownConcept);
        }
        if self.seen.contains(candidate.id()) {
            return Ok(false);
        }
        if self.seen.len() >= self.limits.max_candidates {
            return Err(FrontierError::CandidateLimitReached);
        }

        // The ledger never shrinks and has at most 4096 entries. This ordinal
        // is unique for every admitted identity and cannot overflow.
        let order = self.seen.len();
        let key = (Reverse(candidate.concepts.len()), order);
        self.seen.insert(candidate.id.clone());
        self.pending.insert(key, candidate);
        Ok(true)
    }

    /// Dispatch the highest-priority candidate without performing acquisition.
    ///
    /// Empty takes precedence over exhausted: with no pending work, return
    /// [`FrontierStep::Drained`] even when all attempts were spent. Otherwise
    /// exhaustion leaves the candidate and counter unchanged. Only a returned
    /// [`FrontierStep::Candidate`] consumes one attempt, even if ignored.
    pub fn next_candidate(&mut self) -> FrontierStep {
        if self.pending.is_empty() {
            return FrontierStep::Drained;
        }
        if self.attempts >= self.limits.max_attempts {
            return FrontierStep::BudgetExhausted;
        }
        match self.pending.pop_first() {
            Some((_, candidate)) => {
                self.attempts += 1;
                FrontierStep::Candidate(candidate)
            }
            None => FrontierStep::Drained,
        }
    }

    /// Return the pending candidate count, excluding already dispatched targets.
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Report whether no candidate is pending, not whether collection succeeded.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Return the lifetime dispatch count for this frontier instance.
    pub fn attempts(&self) -> usize {
        self.attempts
    }
}

impl fmt::Debug for CrawlFrontier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CrawlFrontier")
            .field("required_concept_count", &self.required_concepts.len())
            .field("pending_count", &self.pending.len())
            .field("seen_count", &self.seen.len())
            .field("attempts", &self.attempts)
            .field("limits", &self.limits)
            .finish()
    }
}

fn validate_identifier(value: &str) -> Result<(), FrontierError> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || value.chars().any(|ch| ch.is_whitespace() || ch.is_control())
    {
        return Err(FrontierError::InvalidIdentifier);
    }
    Ok(())
}

fn validate_concepts(
    concepts: &[&str],
    require_nonempty: bool,
) -> Result<Vec<String>, FrontierError> {
    if concepts.len() > MAX_CONCEPT_INPUTS || (require_nonempty && concepts.is_empty()) {
        return Err(FrontierError::InvalidConceptSet);
    }
    // Validate all borrowed input before allocating any owned concept strings.
    for &concept in concepts {
        validate_identifier(concept)?;
    }
    let mut seen = BTreeSet::new();
    let mut unique = Vec::with_capacity(concepts.len());
    for &concept in concepts {
        if seen.insert(concept) {
            unique.push(concept.to_owned());
        }
    }
    Ok(unique)
}
