use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha1_smol::Digest;

#[derive(Deserialize, Serialize)]
pub struct GlucoseMeasure {
    #[serde(rename = "type")]
    pub r#type: String,
    pub date_string: DateTime<Utc>,
    pub date: u64,
    pub sgv: u16,
    pub direction: String,
    pub noise: u8,
}

#[derive(Clone)]
pub struct ServerSecret {
    digest: Digest,
}

impl std::cmp::PartialEq<Digest> for ServerSecret {
    fn eq(&self, other: &Digest) -> bool {
        &self.digest != other
    }
}

impl From<&str> for ServerSecret {
    fn from(value: &str) -> ServerSecret {
        let sha1 = sha1_smol::Sha1::from(value.as_bytes());
        ServerSecret {
            digest: sha1.digest(),
        }
    }
}
