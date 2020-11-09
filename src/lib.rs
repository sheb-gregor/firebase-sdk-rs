use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub mod auth;
pub mod errors;
pub mod firebase;

use auth::AuthClient;

/// Version of the Firebase Rust Admin SDK.
pub const VERSION: &'static str = "0.0.1";

const FIREBASE_ENV_NAME: &'static str = "FIREBASE_CONFIG";
const GCLOUD_PROJECT: &'static str = "GCLOUD_PROJECT";
const GOOGLE_CLOUD_PROJECT: &'static str = "GOOGLE_CLOUD_PROJECT";
const GOOGLE_APP_CREDENTIALS: &'static str = "GOOGLE_APPLICATION_CREDENTIALS";

struct ClientOption {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub auth_override: HashMap<String, Value>,
    pub db_url: String,
    pub project_id: String,
    pub service_account_id: String,
    pub storage_bucket: String,
}

#[derive(Clone, Debug, Default)]
pub struct App {
    config: Config,
    opts: Vec<ClientOption>,
}

impl App {
    pub fn new() -> errors::Result<Self> {
        let mut config = read_default_config()?;
        if let Some(p_id) = get_project_by_id() {
            config.project_id = p_id
        }
        Ok(App {
            config,
            opts: Vec::new(),
        })
    }

    pub fn from_config(config: Config) -> Self {
        let mut config = config;
        if let Some(p_id) = get_project_by_id() {
            config.project_id = p_id
        }

        App {
            config,
            opts: Vec::new(),
        }
    }

    pub fn auth(&self) -> AuthClient {}
}

/// Reads the default config file, defined by the FIREBASE_CONFIG
/// env variable.
fn read_default_config() -> errors::Result<Config> {
    let config_file = std::env::var(FIREBASE_ENV_NAME)?;
    let raw_data = std::fs::read(config_file)?;
    let config: Config = serde_json::from_slice(raw_data.as_slice())?;
    Ok(config)

    //todo: handle the `databaseAuthVariableOverride`
}

fn get_project_by_id() -> Option<String> {
    std::env::var(GOOGLE_CLOUD_PROJECT)
        .or_else(|| std::env::var(GCLOUD_PROJECT))
        .ok()
}

/// FirebaseScopes is the set of OAuth2 scopes used by the Admin SDK.
fn firebase_scopes() -> [String; 6] {
    [
        "https://www.googleapis.com/auth/cloud-platform".to_string(),
        "https://www.googleapis.com/auth/datastore".to_string(),
        "https://www.googleapis.com/auth/devstorage.full_control".to_string(),
        "https://www.googleapis.com/auth/firebase".to_string(),
        "https://www.googleapis.com/auth/identitytoolkit".to_string(),
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ]
}
