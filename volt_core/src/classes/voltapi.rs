use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VoltResponse {
    pub version: String,
    #[serde(flatten)]
    pub versions: HashMap<String, VersionData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionData {
    pub packages: HashMap<String, VoltPackage>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VoltPackage {
    pub name: String,
    pub version: String,
    pub tarball: String,
    pub sha1: String,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Vec<String>,
    pub dependencies: Option<Vec<String>>,
    pub bin: Option<HashMap<String, String>>,
}