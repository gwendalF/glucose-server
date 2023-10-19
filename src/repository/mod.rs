use sqlx::{Pool, Postgres};

use crate::{config::AppConfig, entities::GlucoseMeasure};

pub struct Repository {
    database: Pool<Postgres>,
}

impl Repository {
    pub async fn new(config: &AppConfig) -> Result<Repository, ()> {
        let database = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        Ok(Repository { database })
    }

    pub async fn add_measures(
        &self,
        measures: Vec<GlucoseMeasure>,
    ) -> Result<Vec<GlucoseMeasure>, ()> {
        let mut results = Vec::with_capacity(measures.len());
        for measure in measures {
            let handle = tokio::spawn(async move {});
            results.push(handle);
        }
        Ok(vec![])
    }
}
