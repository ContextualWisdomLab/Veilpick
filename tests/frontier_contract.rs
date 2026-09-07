//! Behavioral requirements for the first bounded semantic crawl frontier.

use veilpick::{Candidate, CrawlFrontier, FrontierError, FrontierLimits, FrontierStep};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn frontier(max_candidates: usize, max_attempts: usize) -> Result<CrawlFrontier, FrontierError> {
    CrawlFrontier::new(
        &["urn:product:name", "urn:product:price"],
        FrontierLimits { max_candidates, max_attempts },
    )
}

fn selected(frontier: &mut CrawlFrontier) -> Result<Candidate, Box<dyn std::error::Error>> {
    match frontier.next_candidate() {
        FrontierStep::Candidate(candidate) => Ok(candidate),
        other => Err(format!("expected a candidate, got {other:?}").into()),
    }
}

#[test]
fn prioritizes_explicit_goal_concepts_over_discovery_only_targets() -> TestResult {
    let mut queue = frontier(4, 4)?;
    queue.enqueue(Candidate::new("discovery", &[])?)?;
    queue.enqueue(Candidate::new("name-only", &["urn:product:name"])?)?;
    queue.enqueue(Candidate::new("both", &["urn:product:name", "urn:product:price"])?)?;
    assert_eq!(selected(&mut queue)?.id(), "both");
    assert_eq!(selected(&mut queue)?.id(), "name-only");
    assert_eq!(selected(&mut queue)?.id(), "discovery");
    assert_eq!(queue.attempts(), 3);
    Ok(())
}

#[test]
fn equal_semantic_priority_preserves_discovery_order() -> TestResult {
    let mut queue = frontier(2, 2)?;
    queue.enqueue(Candidate::new("first", &["urn:product:price"])?)?;
    queue.enqueue(Candidate::new("second", &["urn:product:name"])?)?;
    assert_eq!(selected(&mut queue)?.id(), "first");
    assert_eq!(selected(&mut queue)?.id(), "second");
    Ok(())
}

#[test]
fn duplicate_concept_hints_do_not_inflate_priority() -> TestResult {
    let mut queue = frontier(2, 2)?;
    queue.enqueue(Candidate::new("first", &["urn:product:name"])?)?;
    queue.enqueue(Candidate::new("second", &["urn:product:price", "urn:product:price"])?)?;
    assert_eq!(selected(&mut queue)?.id(), "first");
    assert_eq!(selected(&mut queue)?.concepts().len(), 1);
    Ok(())
}

#[test]
fn deduplication_survives_dispatch_and_never_replaces_a_target() -> TestResult {
    let mut queue = frontier(1, 1)?;
    assert!(queue.enqueue(Candidate::new("same", &[])?)?);
    assert!(!queue.enqueue(Candidate::new("same", &["urn:product:price"])?)?);
    assert!(selected(&mut queue)?.concepts().is_empty());
    assert!(!queue.enqueue(Candidate::new("same", &[])?)?);
    assert!(queue.is_empty());
    assert_eq!(queue.attempts(), 1);
    Ok(())
}

#[test]
fn total_attempt_budget_cannot_reset_by_repeated_polling() -> TestResult {
    let mut queue = frontier(2, 1)?;
    queue.enqueue(Candidate::new("first", &[])?)?;
    queue.enqueue(Candidate::new("second", &[])?)?;
    selected(&mut queue)?;
    for _ in 0..20 {
        assert!(matches!(queue.next_candidate(), FrontierStep::BudgetExhausted));
    }
    assert_eq!(queue.attempts(), 1);
    assert_eq!(queue.len(), 1);
    Ok(())
}

#[test]
fn lifetime_unique_target_budget_bounds_the_deduplication_ledger() -> TestResult {
    let mut queue = frontier(1, 2)?;
    queue.enqueue(Candidate::new("first", &[])?)?;
    selected(&mut queue)?;
    assert!(matches!(
        queue.enqueue(Candidate::new("second", &[])?),
        Err(FrontierError::CandidateLimitReached)
    ));
    assert!(queue.is_empty());
    assert_eq!(queue.attempts(), 1);
    Ok(())
}

#[test]
fn unknown_concept_rejection_does_not_poison_target_identity() -> TestResult {
    let mut queue = frontier(1, 1)?;
    assert!(matches!(
        queue.enqueue(Candidate::new("candidate", &["urn:invented"])?),
        Err(FrontierError::UnknownConcept)
    ));
    assert!(queue.enqueue(Candidate::new("candidate", &["urn:product:price"])?)?);
    assert_eq!(queue.len(), 1);
    Ok(())
}

#[test]
fn drained_is_not_a_successful_collection_claim() -> TestResult {
    let mut queue = frontier(1, 1)?;
    assert!(matches!(queue.next_candidate(), FrontierStep::Drained));
    assert_eq!(queue.attempts(), 0);
    queue.enqueue(Candidate::new("candidate", &[])?)?;
    selected(&mut queue)?;
    assert!(matches!(queue.next_candidate(), FrontierStep::Drained));
    Ok(())
}

#[test]
fn invalid_identifiers_fail_before_admission() {
    for value in ["", " ", "has space", "line\nbreak", "\u{0000}"] {
        assert!(matches!(Candidate::new(value, &[]), Err(FrontierError::InvalidIdentifier)));
        assert!(matches!(Candidate::new("valid", &[value]), Err(FrontierError::InvalidIdentifier)));
    }
    let oversized = "a".repeat(257);
    assert!(matches!(Candidate::new(&oversized, &[]), Err(FrontierError::InvalidIdentifier)));
}

#[test]
fn zero_or_excessive_limits_and_empty_goals_fail_closed() {
    for limits in [
        FrontierLimits { max_candidates: 0, max_attempts: 1 },
        FrontierLimits { max_candidates: 1, max_attempts: 0 },
        FrontierLimits { max_candidates: 4097, max_attempts: 1 },
        FrontierLimits { max_candidates: 1, max_attempts: 4097 },
    ] {
        assert!(matches!(CrawlFrontier::new(&["urn:goal"], limits), Err(FrontierError::InvalidLimits)));
    }
    assert!(matches!(
        CrawlFrontier::new(&[], FrontierLimits { max_candidates: 1, max_attempts: 1 }),
        Err(FrontierError::InvalidConceptSet)
    ));
}

#[test]
fn raw_concept_input_count_is_bounded_even_when_all_are_duplicates() {
    let oversized = vec!["urn:goal"; 129];
    assert!(matches!(Candidate::new("valid", &oversized), Err(FrontierError::InvalidConceptSet)));
    assert!(matches!(
        CrawlFrontier::new(&oversized, FrontierLimits { max_candidates: 1, max_attempts: 1 }),
        Err(FrontierError::InvalidConceptSet)
    ));
}

#[test]
fn rejected_enqueue_never_consumes_an_attempt() -> TestResult {
    let mut queue = frontier(1, 1)?;
    let result = queue.enqueue(Candidate::new("target", &["urn:unknown"])?);
    assert!(result.is_err());
    assert_eq!(queue.attempts(), 0);
    assert_eq!(queue.len(), 0);
    Ok(())
}
