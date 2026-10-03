//! Local planning demonstration using synthetic opaque references; no network I/O.

use veilpick::{Candidate, CrawlFrontier, FrontierLimits, FrontierStep};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut frontier = CrawlFrontier::new(
        &["urn:product:name", "urn:product:price"],
        FrontierLimits {
            max_candidates: 8,
            max_attempts: 3,
        },
    )?;
    frontier.enqueue(Candidate::new("catalog-index", &[])?)?;
    frontier.enqueue(Candidate::new("product-name", &["urn:product:name"])?)?;
    frontier.enqueue(Candidate::new(
        "product-detail",
        &["urn:product:name", "urn:product:price"],
    )?)?;

    loop {
        match frontier.next_candidate() {
            FrontierStep::Candidate(candidate) => {
                println!("selected synthetic target: {}", candidate.id());
            }
            FrontierStep::Drained => {
                println!("frontier drained; collection success still requires evidence");
                break;
            }
            FrontierStep::BudgetExhausted => {
                println!("dispatch budget exhausted; pending targets were preserved");
                break;
            }
        }
    }
    Ok(())
}
