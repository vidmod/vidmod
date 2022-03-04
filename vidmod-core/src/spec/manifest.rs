use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectManifest {
    pub nodes: BTreeMap<String, ManifestNode>,
    pub links: Vec<ManifestLink>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestNode {
    pub name: String,
    #[serde(default)]
    pub args: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestLink {
    pub from: (String, String),
    pub to:   (String, String),
}
