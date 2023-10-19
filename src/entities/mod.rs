use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use sha1_smol::Digest;

use self::errors::AppError;

pub mod errors;

#[derive(Deserialize)]
pub struct GetDataQuery {
    pub from: DateTime<FixedOffset>,
    pub to: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize)]
pub struct GlucoseMeasure {
    #[serde(rename = "type")]
    pub raw_type: String,
    #[serde(rename = "date_string")]
    pub date: DateTime<Utc>,
    #[serde(rename = "date")]
    pub timestamp: u64,
    pub sgv: u16,
    pub direction: String,
    pub noise: u8,
}

pub struct DatabaseMeasure {
    pub raw_type: String,
    pub date: DateTime<Utc>,
    pub timestamp: i64,
    pub sgv: i32,
    pub direction: String,
    pub noise: i32,
}

impl TryFrom<DatabaseMeasure> for GlucoseMeasure {
    type Error = AppError;
    fn try_from(value: DatabaseMeasure) -> Result<Self, Self::Error> {
        let timestamp = u64::try_from(value.timestamp)?;
        let noise = u8::try_from(value.noise)?;
        let sgv = u16::try_from(value.sgv)?;
        Ok(GlucoseMeasure {
            noise,
            timestamp,
            sgv,
            raw_type: value.raw_type,
            date: value.date,
            direction: value.direction,
        })
    }
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
