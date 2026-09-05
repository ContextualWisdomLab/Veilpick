//! Boundary, state-integrity, and diagnostic requirements for the crawl frontier.

use veilpick::{Candidate, CrawlFrontier, FrontierError, FrontierLimits, FrontierStep};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn limits(max_candidates: usize, max_attempts: usize) -> FrontierLimits {
    FrontierLimits {
        max_candidates,
        max_attempts,
    }
}

fn frontier(max_candidates: usize, max_attempts: usize) -> Result<CrawlFrontier, FrontierError> {
    CrawlFrontier::new(&["urn:name", "urn:price"], limits(max_candidates, max_attempts))
}

fn selected(queue: &mut CrawlFrontier) -> Result<Candidate, Box<dyn std::error::Error>> {
    match queue.next_candidate() {
        FrontierStep::Candidate(candidate) => Ok(candidate),
        other => Err(format!("expected candidate, got {other:?}").into()),
    }
}

#[test]
fn identifier_limits_count_utf8_bytes_not_characters() -> TestResult {
    let exact = format!("{}a", "한".repeat(85));
    assert_eq!(exact.len(), 256);
    let candidate = Candidate::new(&exact, &[&exact])?;
    let mut queue = CrawlFrontier::new(&[&exact], limits(1, 1))?;
    assert!(queue.enqueue(candidate)?);
    assert_eq!(selected(&mut queue)?.id(), exact);

    for oversized in ["x".repeat(257), "한".repeat(86)] {
        assert!(matches!(
            Candidate::new(&oversized, &[]),
            Err(FrontierError::InvalidIdentifier)
        ));
        assert!(matches!(
            Candidate::new("valid", &[&oversized]),
            Err(FrontierError::InvalidIdentifier)
        ));
        assert!(matches!(
            CrawlFrontier::new(&[&oversized], limits(1, 1)),
            Err(FrontierError::InvalidIdentifier)
        ));
    }
    Ok(())
}

#[test]
fn unicode_whitespace_and_controls_are_rejected_in_every_identifier_position() {
    for value in ["", "a\u{00a0}b", "a\u{3000}b", "a\u{007f}b", "a\u{0080}b", "\t"] {
        assert!(matches!(
            Candidate::new(value, &[]),
            Err(FrontierError::InvalidIdentifier)
        ));
        assert!(matches!(
            Candidate::new("valid", &[value]),
            Err(FrontierError::InvalidIdentifier)
        ));
        assert!(matches!(
            CrawlFrontier::new(&[value], limits(1, 1)),
            Err(FrontierError::InvalidIdentifier)
        ));
    }
}

#[test]
fn concept_input_at_the_exact_raw_limit_is_admitted_before_deduplication() -> TestResult {
    let repeated = ["urn:name"; 128];
    let mut queue = CrawlFrontier::new(&repeated, limits(1, 1))?;
    let candidate = Candidate::new("target", &repeated)?;
    assert_eq!(candidate.concepts().len(), 1);
    assert!(queue.enqueue(candidate)?);
    assert_eq!(selected(&mut queue)?.concepts().len(), 1);
    Ok(())
}

#[test]
fn required_goal_duplicates_do_not_amplify_a_candidates_priority() -> TestResult {
    let mut queue = CrawlFrontier::new(&["urn:name", "urn:name", "urn:price"], limits(2, 2))?;
    queue.enqueue(Candidate::new("price-first", &["urn:price"])?)?;
    queue.enqueue(Candidate::new("name-second", &["urn:name"])?)?;
    assert_eq!(selected(&mut queue)?.id(), "price-first");
    assert_eq!(selected(&mut queue)?.id(), "name-second");
    Ok(())
}

#[test]
fn candidate_concepts_preserve_first_occurrence_order() -> TestResult {
    let candidate = Candidate::new("target", &["urn:price", "urn:name", "urn:price"])?;
    let actual: Vec<&str> = candidate.concepts().iter().map(String::as_str).collect();
    assert_eq!(actual, ["urn:price", "urn:name"]);
    Ok(())
}

#[test]
fn opaque_identities_are_not_case_folded_or_unicode_normalized() -> TestResult {
    let mut queue = frontier(4, 4)?;
    let ids = ["Page", "page", "é", "e\u{0301}"];
    for id in ids {
        assert!(queue.enqueue(Candidate::new(id, &[])?)?);
    }
    for id in ids {
        assert_eq!(selected(&mut queue)?.id(), id);
    }
    Ok(())
}

#[test]
fn draining_then_refilling_preserves_the_lifetime_attempt_counter() -> TestResult {
    let mut queue = frontier(3, 2)?;
    for id in ["first", "second"] {
        queue.enqueue(Candidate::new(id, &[])?)?;
        assert_eq!(selected(&mut queue)?.id(), id);
        assert!(matches!(queue.next_candidate(), FrontierStep::Drained));
    }
    queue.enqueue(Candidate::new("third", &[])?)?;
    for _ in 0..5 {
        assert!(matches!(
            queue.next_candidate(),
            FrontierStep::BudgetExhausted
        ));
    }
    assert_eq!(queue.attempts(), 2);
    assert_eq!(queue.len(), 1);
    Ok(())
}

#[test]
fn unknown_hints_are_checked_before_duplicate_or_capacity_shortcuts() -> TestResult {
    let mut queue = frontier(1, 1)?;
    queue.enqueue(Candidate::new("first", &["urn:name"])?)?;
    for id in ["first", "other"] {
        assert!(matches!(
            queue.enqueue(Candidate::new(id, &["urn:unknown"])?),
            Err(FrontierError::UnknownConcept)
        ));
    }
    assert!(matches!(
        queue.enqueue(Candidate::new("other", &["urn:price"])?),
        Err(FrontierError::CandidateLimitReached)
    ));
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.attempts(), 0);
    let retained = selected(&mut queue)?;
    assert_eq!(retained.id(), "first");
    assert_eq!(retained.concepts(), &["urn:name".to_owned()]);
    Ok(())
}

#[test]
fn interleaved_dispatch_does_not_reuse_a_pending_order_key() -> TestResult {
    let mut queue = frontier(4, 4)?;
    queue.enqueue(Candidate::new("discovery", &[])?)?;
    queue.enqueue(Candidate::new("first-match", &["urn:name"])?)?;
    queue.enqueue(Candidate::new("second-match", &["urn:price"])?)?;
    assert_eq!(selected(&mut queue)?.id(), "first-match");
    queue.enqueue(Candidate::new("third-match", &["urn:name"])?)?;
    for expected in ["second-match", "third-match", "discovery"] {
        assert_eq!(selected(&mut queue)?.id(), expected);
    }
    assert_eq!(queue.attempts(), 4);
    assert!(matches!(queue.next_candidate(), FrontierStep::Drained));
    Ok(())
}

#[test]
fn all_small_priority_sequences_match_an_independent_sorted_oracle() -> TestResult {
    let hints: [&[&str]; 3] = [&[], &["urn:name", "urn:name"], &["urn:price", "urn:name"]];
    for encoded in 0..81_usize {
        let mut queue = frontier(4, 4)?;
        let mut digits = encoded;
        let mut expected = Vec::new();
        for index in 0..4 {
            let priority = digits % 3;
            digits /= 3;
            // Reverse lexical IDs so lexical sorting cannot masquerade as FIFO.
            let id = format!("target-{}", 3 - index);
            queue.enqueue(Candidate::new(&id, hints[priority])?)?;
            expected.push((2 - priority, index, id));
        }
        expected.sort();
        for (_, _, id) in expected {
            assert_eq!(selected(&mut queue)?.id(), id, "sequence {encoded}");
        }
        assert_eq!(queue.attempts(), 4);
        assert!(matches!(queue.next_candidate(), FrontierStep::Drained));
    }
    Ok(())
}

#[test]
fn maximum_limits_are_inclusive_and_the_seen_ledger_remains_bounded() -> TestResult {
    let mut queue = frontier(4096, 4096)?;
    for index in 0..4096 {
        assert!(queue.enqueue(Candidate::new(&format!("target-{index}"), &[])?)?);
    }
    assert_eq!(queue.len(), 4096);
    assert!(matches!(
        queue.enqueue(Candidate::new("overflow", &[])?),
        Err(FrontierError::CandidateLimitReached)
    ));
    for index in 0..4096 {
        assert_eq!(selected(&mut queue)?.id(), format!("target-{index}"));
    }
    assert_eq!(queue.attempts(), 4096);
    assert!(matches!(queue.next_candidate(), FrontierStep::Drained));
    assert!(!queue.enqueue(Candidate::new("target-0", &[])?)?);
    assert!(matches!(
        queue.enqueue(Candidate::new("new-after-drain", &[])?),
        Err(FrontierError::CandidateLimitReached)
    ));
    for invalid in [limits(usize::MAX, 1), limits(1, usize::MAX)] {
        assert!(matches!(
            CrawlFrontier::new(&["urn:name"], invalid),
            Err(FrontierError::InvalidLimits)
        ));
    }
    Ok(())
}

#[test]
fn debug_output_does_not_disclose_opaque_ids_or_concepts() -> TestResult {
    let id = "private-target-reference";
    let concept = "urn:private:concept";
    let candidate = Candidate::new(id, &[concept])?;
    let candidate_debug = format!("{candidate:?}");
    let mut queue = CrawlFrontier::new(&[concept], limits(1, 1))?;
    queue.enqueue(candidate)?;
    let queue_debug = format!("{queue:?}");
    let step_debug = format!("{:?}", queue.next_candidate());
    for diagnostic in [candidate_debug, queue_debug, step_debug] {
        assert!(!diagnostic.contains(id));
        assert!(!diagnostic.contains(concept));
    }
    Ok(())
}

#[test]
fn every_error_has_a_static_non_reflecting_display_message() {
    let cases = [
        (FrontierError::InvalidIdentifier, "invalid frontier identifier"),
        (FrontierError::InvalidConceptSet, "invalid frontier concept set"),
        (FrontierError::InvalidLimits, "invalid frontier limits"),
        (FrontierError::UnknownConcept, "candidate has an undeclared concept"),
        (FrontierError::CandidateLimitReached, "frontier lifetime candidate limit reached"),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }
}
