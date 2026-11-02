//! This module implements an EOL information fetcher which downloads
//! the end of support date from endoflife.date.
//! https://endoflife.date/docs/api/v1/

use crate::models::{EOLInfo, Finding};
use crate::support::cache_managers::EOLCacheManager;
use crate::support::fetchers::EOLFetcher;
use log::{error, trace};
use serde::Deserialize;

pub struct EndoflifedateFetcher {
    /// An optional cache manager to reduce API calls.
    cache: Option<Box<dyn EOLCacheManager>>,
}

impl EndoflifedateFetcher {
    /// Complete the EOL information for a finding.
    pub fn fetch_eol(&self, finding: &mut Finding) {
        trace!("Running EndoflifedateFetcher::fetch_eol()");
        let techno_name = if let Some(n) = finding.technology.get_endoflifedate_name() {
            n
        } else {
            return;
        };

        if finding.version.is_none() {
            return;
        }
        
        let url = format!("https://endoflife.date/api/v1/products/{}", techno_name);
        let response_res = reqwest::blocking::get(url);
        let response = if let Err(e) = response_res {
            error!("Error while communicating with endoflife.date. {:?}", e);
            return;
        } else {
            response_res.unwrap()
        };

        if !response.status().is_success() {
            error!("Invalid HTTP response code: {}", response.status());
            return;
        }

        let response_json_res = response.json();
        let eol_response: APIResponse = if let Err(e) = response_json_res {
            error!("Invalid JSON returned by endoflife.date API. {:?}", e);
            return;
        } else {
            response_json_res.unwrap()
        };

        let release = finding.get_technology_release();
        if release.is_none() {
            return;
        }
        let release = release.unwrap();

        for eol_release in eol_response.result.releases {
            if release == eol_release.name {
                if eol_release.is_eol && eol_release.eol_from.is_some() {
                    finding.end_of_life = Some(EOLInfo::new(&eol_release.eol_from.unwrap()));
                    break;
                }
            }
        }

        if let Some(c) = self.cache.as_ref() && finding.end_of_life.is_some() {
            c.store(
                finding.end_of_life.clone().unwrap(),
                finding.technology.clone(),
                &release
            );
        }
        
    }
}

impl EOLFetcher for EndoflifedateFetcher {
    /// Creates a new EndoflifedateFetcher.
    fn new(cache: Option<Box<dyn EOLCacheManager>>) -> Self {
        Self { cache }
    }

    /// Complete the findings with EOL info coming from endoflife.date.
    fn complete_findings(&self, findings: &mut Vec<Finding>) {
        for finding in findings {
            let mut cache_found = false;
            if self.cache.is_some() {
                cache_found = self.cache.as_ref().unwrap().complete_finding(finding);
            }
            if !cache_found {
                self.fetch_eol(finding);
            }
        }
    }
}

/// Represents the main JSON object returned by the endoflife.date API
/// when querying /api/v1/products/<product>.
#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub schema_version: String,
    /// datetime
    pub generated_at: String,
    /// datetime
    pub last_modified: String,
    pub result: Result,
}

/// Represents the result of the request.
#[derive(Debug, Deserialize)]
pub struct Result {
    /// The name of the technology, e.g. jquery.
    pub name: String,
    /// Alternative names for the technologies.
    /// For example Apache httpd has name "apache-http-server" and
    /// "apache" and "httpd" in aliases.
    pub aliases: Vec<String>,
    /// The public name, like jQuery or Apache HTTP Server.
    pub label: String,
    pub category: String,
    pub tags: Vec<String>,
    /// The command used to get the version, e.g. a SQL query for MariaDB.
    #[serde(rename = "versionCommand")]
    pub version_command: Option<String>,
    pub identifiers: Vec<Identifier>,
    pub labels: Labels,
    pub links: Links,
    pub releases: Vec<Release>,
}

/// An identifier is an identifier for the technology repository or package.
/// Example: pkg:github/jquery/jquery.
#[derive(Debug, Deserialize)]
pub struct Identifier {
    /// The type of identifier, like "purl" or "repology".
    #[serde(rename = "type")]
    pub type_: String,
    /// The identifier.
    pub id: String,
}

/// Represents labels about a technology.
/// They are the table headers displayed on the Web page.
/// A value of None means the column isn't used.
#[derive(Debug, Deserialize)]
pub struct Labels {
    /// End of active support
    pub eoas: Option<String>,
    /// Discountinued.
    pub discontinued: Option<String>,
    /// End of life.
    pub eol: Option<String>,
    /// End of extended support.
    pub eoes: Option<String>,
}

/// Represents links about a technology.
#[derive(Debug, Deserialize)]
pub struct Links {
    pub icon: Option<String>,
    pub html: String,
    #[serde(rename = "releasePolicy")]
    pub release_policy: Option<String>,
}

/// Represent a specific release for a technology.
#[derive(Debug, Deserialize)]
pub struct Release {
    /// The name of a release, e.g. "26.04" for Ubuntu.
    pub name: String,
    /// The codename, e.g. "Resolute Raccoon" for Ubuntu 26.04.
    pub codename: Option<String>,
    /// The label, e.g. "26.04 'Resolute Raccoon' (LTS)"
    pub label: String,
    /// The date when it was released.
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    /// Whether the release is in Long-Term Support (LTS).
    #[serde(rename = "isLts")]
    pub is_lts: bool,
    /// The date from when the release became LTS.
    #[serde(rename = "ltsFrom")]
    pub lts_from: Option<String>,
    /// Whether the release reached end of active support.
    #[serde(rename = "isEoas")]
    pub is_eoas: Option<bool>,
    /// The date from when the release reached end of active support.
    #[serde(rename = "eoasFrom")]
    pub eoas_from: Option<String>,
    /// Whether the release reached end of life.
    #[serde(rename = "isEol")]
    pub is_eol: bool,
    /// The date from when the release reached end of life.
    #[serde(rename = "eolFrom")]
    pub eol_from: Option<String>,
    #[serde(rename = "isDiscontinued")]
    pub is_discontinued: Option<bool>,
    #[serde(rename = "discontinuedFrom")]
    pub discontinued_from: Option<String>,
    /// Whether the release reached extended support.
    #[serde(rename = "isEoes")]
    pub is_eoes: Option<bool>,
    #[serde(rename = "eoesFrom")]
    /// The date from when the release reached extended support.
    pub eoes_from: Option<String>,
    /// Whether the release is maintained.
    #[serde(rename = "isMaintained")]
    pub is_maintained: bool,
    /// The infos about the latest release for this technology.
    pub latest: Option<ReleaseLatest>,
    pub custom: Option<ReleaseCustom>,
}

/// Infos about the latest release for a technology.
#[derive(Debug, Deserialize)]
pub struct ReleaseLatest {
    /// The name of the release.
    pub name: String,
    /// Date of release.
    pub date: String,
    /// Link to the release, e.g. its Git repository.
    pub link: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseCustom {
    #[serde(rename = "chromeVersion")]
    pub chrome_version: Option<String>,
    #[serde(rename = "nodeVersion")]
    pub node_version: Option<String>,
    #[serde(rename = "technical_guidance")]
    pub technical_guidance: Option<String>,
    #[serde(rename = "kernel_version")]
    pub kernel_version: Option<String>,
    /// Examples: "5.2, 6.0", "7.4 - 8.5"
    #[serde(rename = "supportedDjangoVersions")]
    pub supported_django_versions: Option<String>,
    /// Examples: "5.2, 6.0", "7.4 - 8.5"
    #[serde(rename = "supportedPythonVersions")]
    pub supported_python_versions: Option<String>,
    /// Examples: "5.2, 6.0", "7.4 - 8.5"
    #[serde(rename = "supportedPHPVersions")]
    pub supported_php_versions: Option<String>,
    #[serde(rename = "ubuntuVersion")]
    pub ubuntu_version: Option<String>,
    #[serde(rename = "pythonVersion")]
    pub python_version: Option<String>,
    #[serde(rename = "sparkVersion")]
    pub spark_version: Option<String>,
    #[serde(rename = "javaVersion")]
    pub java_version: Option<String>,
    #[serde(rename = "upgradeVersion")]
    pub upgrade_version: Option<String>,
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "eoesProvider")]
    pub eoes_provider: Option<String>,
    #[serde(rename = "pythonVersionsControlNode")]
    pub python_versions_control_node: Option<String>,
    #[serde(rename = "powershellVersionsManagedNode")]
    pub powershell_versions_managed_node: Option<String>,
    #[serde(rename = "ansibleCoreVersion")]
    pub ansible_core_version: Option<String>,
    #[serde(rename = "recommendedReplacement")]
    pub recommended_replacement: Option<String>,
    #[serde(rename = "supportedJavaVersions")]
    pub supported_java_versions: Option<String>,
    #[serde(rename = "recommendedJavaVersion")]
    pub recommended_java_version: Option<String>,
    #[serde(rename = "supportedWatchOsVersions")]
    pub supported_watch_os_versions: Option<String>,
    #[serde(rename = "latestJdkVersion")]
    pub latest_jdk_version: Option<String>,
}

