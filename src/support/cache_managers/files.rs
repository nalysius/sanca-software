//! This module contains the FileCacheManager struct.
use crate::models::{EOLInfo, Finding, technology::Technology};
use crate::support::cache_managers::EOLCacheManager;
use log::{error, trace};
use std::env;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// Represents a cache manager which stores the data in files.
pub struct EOLFileCacheManager {}

impl EOLFileCacheManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl EOLCacheManager for EOLFileCacheManager {
    /// Complete the finding with EOL information stored in files.
    /// Returns true if the technology & version in the finding were
    /// stored in cache, false otherwise.
    fn complete_finding(&self, finding: &mut Finding) -> bool {
        if finding.version.is_none() {
            return false;
        }
        let release = finding.get_technology_release();
        if release.is_none() {
            return false;
        }
        let release = release.unwrap();

        if let Some(eol) = self.read(finding.technology.clone(), &release) {
            finding.end_of_life = Some(eol);
            return true;
        } else {
            return false;
        }
    }

    /// Read the EOL information associated with a Technology & release in cache.
    fn read(&self, technology: Technology, release: &str) -> Option<EOLInfo> {
        trace!("Running EOLFileCacheManager::read");
        let root_dir = if let Ok(mut p) = env::current_exe() {
            p.pop();
            p.join("eols")
        } else {
            // By default, a "eols" directory is created in the current working directory
            PathBuf::new().join("eols")
        };

        let (_part, vendor, product) = technology.get_cpe_part_vendor_product();
        let dirname = Path::new(&root_dir).join(vendor).join(product);
        if !dirname.exists() {
            return None;
        }

        let filename = dirname.join(&format!("{}.json", release));
        if !filename.exists() {
            return None;
        }

        let mut file = if let Ok(f) = File::open(filename.clone()) {
            f
        } else {
            error!(
                "Unable to open the JSON file {}",
                filename.to_string_lossy()
            );
            return None;
        };

        let mut json = String::new();
        if file.read_to_string(&mut json).is_err() {
            error!(
                "Unable to read the JSON file {}",
                filename.to_string_lossy()
            );
            return None;
        }

        let eol: EOLInfo = if let Ok(c) = serde_json::from_str(json.as_str()) {
            c
        } else {
            return None;
        };

        return Some(eol);
    }

    /// Stores the EOL information associated with a Technology & version in cache.
    ///
    /// The cache has the following structure:
    /// eols/
    ///   \__<cpe_vendor>/
    ///        \__<cpe_product>
    ///              \__<release>.json
    ///
    /// The JSON files contain an EOLInfo.
    fn store(&self, eol: EOLInfo, technology: Technology, release: &str) {
        trace!("Running EOLFileCacheManager::store");
        let root_dir = if let Ok(mut p) = env::current_exe() {
            p.pop();
            p.join("eols")
        } else {
            // By default, a "eols" directory will be created in the current working directory
            PathBuf::new().join("eols")
        };

        let eol_json = if let Ok(j) = serde_json::value::to_value(eol) {
            format!("{:#}", j)
        } else {
            error!("Error while serializing a EOLInfo to a JSON string");
            return;
        };

        let (_part, vendor, product) = technology.get_cpe_part_vendor_product();
        let dirname = Path::new(&root_dir).join(vendor).join(product);
        if !dirname.exists() {
            let create_result = create_dir_all(&dirname);
            if create_result.is_err() {
                error!(
                    "Unable to create the directory {}",
                    dirname.to_string_lossy()
                );
                return;
            }
        }
        let filename = dirname.join(&format!("{}.json", release));
        if !filename.exists() {
            let mut file = if let Ok(f) = File::create(filename.clone()) {
                f
            } else {
                error!("Unable to create the file {}", filename.to_string_lossy());
                return;
            };

            if file.write_all(eol_json.as_bytes()).is_err() {
                error!("Unable to write the file {}", filename.to_string_lossy());
            }
        }
    }
}
