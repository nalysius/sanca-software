//! The module support::cache_managers defines the cache managers for EOL information.
pub mod files;

use crate::models::{EOLInfo, Finding, technology::Technology};

/// A common interface between all cache managers.
pub trait EOLCacheManager {
    /// Completes the finding with EOL information stored in cache.
    /// Returns true if the technology & version in the finding were
    /// stored in cache, false otherwise.
    fn complete_finding(&self, finding: &mut Finding) -> bool;

    /// Read the EOL information associated with a Technology & release in cache.
    fn read(&self, technology: Technology, release: &str) -> Option<EOLInfo>;

    /// Stores the EOL information associated with a Technology & release in cache.
    fn store(&self, eol: EOLInfo, technology: Technology, release: &str);
}
