use std::sync::Arc;

use crate::{config::AppConfig, entities::ServerSecret, repository::Repository};
mod check_auth;

pub use check_auth::auth_middleware;

pub struct Service {
    pub repo: Repository,
    access_secret: ServerSecret,
}

impl Service {
    pub async fn from_config(config: &AppConfig) -> Result<AppState, ()> {
        let sercret = ServerSecret::from(config.access_secret.as_str());
        let repo = Repository::new(config).await.unwrap();
        Ok(Arc::new(Service {
            repo,
            access_secret: sercret,
        }))
    }
}

pub type AppState = Arc<Service>;
