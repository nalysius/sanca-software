//! In this module are declared the entities manipulated by this program

pub mod reqres;
pub mod technology;

use crate::vulnerabilities::fetchers::nvd::Vulnerability;
use clap::{builder::PossibleValue, ValueEnum};
use serde::{Deserialize, Serialize};
use std::convert::From;
use technology::Technology;

/// Represents the type of scan
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScanType {
    /// Protocol TCP
    Tcp,
    /// Protocol UDP (not supported yet)
    Udp,
    /// Protocol HTTP
    Http,
}

impl ValueEnum for ScanType {
    /// Lists the variants available for clap
    fn value_variants<'a>() -> &'a [Self] {
        &[ScanType::Tcp, ScanType::Http, ScanType::Udp]
    }

    /// Map each value to a possible value in clap
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match &self {
            ScanType::Tcp => Some(PossibleValue::new("tcp")),
            ScanType::Http => Some(PossibleValue::new("http")),
            ScanType::Udp => Some(PossibleValue::new("udp")),
        }
    }
}

/// Represents a finding of a technology running on an asset
#[derive(Clone, Serialize)]
pub struct Finding {
    /// The technology found
    pub technology: Technology,
    /// The version of the technology
    /// Optional since it can be unknown
    pub version: Option<String>,
    /// The evidence of the finding
    pub evidence: String,
    /// The text for the evidence
    pub evidence_text: String,
    /// The URL where the finding has been found.
    pub url_of_finding: Option<String>,
    /// The list of vulnerabilities.
    pub vulnerabilities: Vec<CVE>,
    /// The date when the version of the technology reached end of life.
    pub end_of_life: Option<EOLInfo>,
}

impl Finding {
    /// Creates a new finding
    pub fn new(
        technology: Technology,
        version: Option<&str>,
        evidence: &str,
        evidence_text: &str,
        url_of_finding: Option<&str>,
    ) -> Self {
        Finding {
            technology: technology,
            version: version.map(|f| f.to_string()),
            evidence: evidence.to_string(),
            evidence_text: evidence_text.to_string(),
            url_of_finding: url_of_finding.map(|f| f.to_string()),
            vulnerabilities: Vec::new(),
            end_of_life: None,
        }
    }

    /// Extracts the release for endoflife.date from the version.
    ///
    /// From a technology version we need to know the supported release,
    /// e.g. 3.x, 8.2.x and so on. It can be different for each technology,
    /// jQuery supports the major version, like 3 when version is 3.7.1, but
    /// PHP supports 8.5 when version is 8.5.2.
    ///
    /// # Parameters
    ///
    /// nbparts is the number of version parts to include. If version is 3.7.1 and
    /// nbparts is 1, the function will return 3. If nbparts is 2, the function
    /// will return 3.7.
    ///
    /// # Return
    ///
    /// (release, was_long_enough). The first item of the tuple is the release.
    /// If the version wasn't long enough to extract nbparts, the complete version
    /// is returned.
    /// The second item of the tuple is true if the version was long enough to
    /// extract the release, false otherwise.
    fn extract_release(&self, nbparts: usize) -> (String, bool) {
        if self.version.is_none() {
            return (String::new(), false)
        }
        let version = self.version.clone().unwrap();
        let parts: Vec<&str> = version.split(".").collect();
        if parts.len() <= nbparts {
            return (version.to_string(), false);
        }

        let mut release: String = String::new();
        let mut counter: usize = 0;
        for p in parts {
            if counter == nbparts {
                break;
            }
            if release.len() == 0 {
                release = format!("{}", p);
            } else {
                release = format!("{}.{}", release, p);
            }
            counter += 1;
        }

        (release, true)
    }

    /// Gets the release of the technology.
    /// It's used by endoflife.date, it's not the complete version but is used to
    /// determine if it's supported. For example, jQuery uses major versions like
    /// 1, 2, 3 or 4, while PHP uses 8.3, 8.4, 8.5.
    pub fn get_technology_release(&self) -> Option<String> {
        if self.version.is_none() {
            return None;
        }
        let nbparts = match self.technology {
            Technology::Bootstrap => 1,
            Technology::JQuery => 1,
            Technology::Typo3 => 1,
            _ => 2, // Most technologies use 2 parts.
        };
        let (release, _long_enough) = self.extract_release(nbparts);
        Some(release)
    }
}

impl PartialEq for Finding {
    /// Two findings are equal if their technology and version are the same
    fn eq(&self, other: &Self) -> bool {
        return self.technology == other.technology && self.version == other.version;
    }
}

/// An enum to match the available writers
#[derive(Clone, Debug, PartialEq)]
pub enum Writers {
    /// StdoutWriter
    TextStdout,
    /// CsvWriter
    Csv,
    /// JsonWriter
    Json,
}

impl ValueEnum for Writers {
    /// Lists the variants available for clap
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::TextStdout, Self::Csv, Self::Json]
    }

    /// Map each value to a possible value in clap
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match &self {
            Self::TextStdout => Some(PossibleValue::new("textstdout")),
            Self::Csv => Some(PossibleValue::new("csv")),
            Self::Json => Some(PossibleValue::new("json")),
        }
    }
}

/// Represents a CVE, as embedded in a Finding.
#[derive(Clone, Deserialize, Serialize)]
pub struct CVE {
    /// The CVE identifier.
    /// Example: CVE-2012-6708
    pub cve_id: String,
    /// The base score.
    /// Example: 8.3
    pub base_score: f64,
    /// The CVSS version.
    pub cvss_version: String,
}

/// Implements from Vulnerability instead of NVD CVE, because it's
/// upper in the NVD response hierarchy and will be simpler to call.
impl From<Vulnerability> for CVE {
    fn from(vuln: Vulnerability) -> Self {
        let cve_id = vuln.cve.id.clone();
        let metrics = vuln.cve.metrics;
        let mut base_score: f64 = 0.0;
        let mut cvss_version = String::new();
        if let Some(metric) = metrics.cvss_metric_v31 {
            if !metric.is_empty() {
                base_score = metric[0].cvss_data.base_score;
                cvss_version = "3.1".to_string();
            }
        } else if let Some(metric) = metrics.cvss_metric_v30 {
            if !metric.is_empty() {
                base_score = metric[0].cvss_data.base_score;
                cvss_version = "3.0".to_string();
            }
        } else if let Some(metric) = metrics.cvss_metric_v2 {
            if !metric.is_empty() {
                base_score = metric[0].cvss_data.base_score;
                cvss_version = "2".to_string();
            }
        }

        Self {
            cve_id,
            base_score,
            cvss_version,
        }
    }
}

impl PartialEq for CVE {
    /// Two CVE are equal if they have the same identifier.
    fn eq(&self, other: &Self) -> bool {
        return self.cve_id == other.cve_id;
    }
}

/// Represents information about end of life of a technology.
#[derive(Clone, Deserialize, Serialize)]
pub struct EOLInfo {
    /// The date when the technology reached end of life.
    pub date: String,
}

impl EOLInfo {
    /// Creates a new EOLInfo
    pub fn new(date: &str) -> Self {
        Self {
            date: date.to_string(),
        }
    }
}
