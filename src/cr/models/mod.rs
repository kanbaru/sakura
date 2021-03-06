use serde::{Serialize, Deserialize};
pub mod search;
pub mod seasons;
pub mod episodes;
pub mod stream;
pub mod series;
#[derive(Debug, Serialize, Deserialize)]
pub struct CrApiCms {
    pub cms: Cms,
    pub service_available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cms {
    pub bucket: String,
    pub policy: String,
    pub signature: String,
    pub key_pair_id: String,
    pub expires: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrApiAccessToken {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub country: String,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VrvAccessToken {
    signing_policies: Vec<SigningPolicy>,
    service_available: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SigningPolicy {
    name: String,
    path: String,
    value: String,
    expires: String,
}