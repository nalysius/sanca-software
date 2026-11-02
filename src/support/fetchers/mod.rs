//! The module fetchers defines the EOL information fetchers.

pub mod endoflifedate;

use crate::models::Finding;
use crate::support::cache_managers::EOLCacheManager;

/// A common interface between all EOL information fetchers.
pub trait EOLFetcher {
    /// Creates a new instance of the fetcher.
    fn new(cache: Option<Box<dyn EOLCacheManager>>) -> Self;

    /// Complete the findings with the EOL information.
    fn complete_findings(&self, findings: &mut Vec<Finding>);
}
