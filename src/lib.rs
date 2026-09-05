//! Veilpick's semantic acquisition core.
//!
//! The bounded crawl frontier selects opaque acquisition references using
//! explicit task-concept hints, lifetime deduplication, and dispatch budgets.
//! Selection is not acquisition authority, extraction evidence, or success.
//!
//! Network access, applied stealth, ontology induction, extraction, and automatic
//! challenge resolution are separate required product capabilities, not supplied
//! by this first planning primitive.

mod frontier;

pub use frontier::{Candidate, CrawlFrontier, FrontierError, FrontierLimits, FrontierStep};
