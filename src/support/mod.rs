//! The support module contains what's needed to determine if a technology
//! identified in a finding has reached end of support.
//!
//! It is composed of two parts, just like the vulnerabilities module, fetchers
//! and cache managers.
//! A fetcher is there to fetch the end of life (EOL) information from a source
//! (e.g. endoflife.date).
//! A cache manager stored the vulnerabilities fetched y a fetcher (e.g. in file
//! or database).

pub mod cache_managers;
pub mod fetchers;

use clap::{ValueEnum, builder::PossibleValue};

/// This enum represents the source of the vulnerabilities.
#[derive(Clone, Debug)]
pub enum EOLSource {
    /// The EOL information are downloaded from the endoflife.date.
    Endoflifedate,
}

impl ValueEnum for EOLSource {
    /// Lists the variants available for clap
    fn value_variants<'a>() -> &'a [Self] {
        &[EOLSource::Endoflifedate]
    }

    /// Map each value to a possible value in clap
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match &self {
            EOLSource::Endoflifedate => Some(PossibleValue::new("endoflifedate")),
        }
    }
}

/// This enum represents the type of cache that can be used.
#[derive(Clone, Debug)]
pub enum EOLCacheType {
    /// The cache will store the EOL information in files.
    Files,
}

impl ValueEnum for EOLCacheType {
    /// Lists the variants available for clap
    fn value_variants<'a>() -> &'a [Self] {
        &[EOLCacheType::Files]
    }

    /// Map each value to a possible value in clap
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match &self {
            EOLCacheType::Files => Some(PossibleValue::new("files")),
        }
    }
}
