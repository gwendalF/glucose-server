use sqlx::{Pool, Postgres};

use crate::{
    config::AppConfig,
    entities::{errors::AppResult, GlucoseMeasure},
};

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
    ) -> AppResult<Vec<GlucoseMeasure>> {
        // Need to bench against await in loop
        let mut task_set = tokio::task::JoinSet::new();
        for measure in measures {
            let task_db = self.database.clone();
            task_set.spawn(async move {
                match sqlx::query!(
                    r#"
                INSERT INTO cgm_values (time, type, date, sgv, direction, noise) 
                VALUES ($1, $2, $3, $4, $5, $6)"#,
                    measure.date_string,
                    measure.r#type,
                    measure.date as i64,
                    measure.sgv as i32,
                    measure.direction,
                    measure.noise as i32
                )
                .execute(&task_db)
                .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(measure),
                }
            });
        }
        let mut failed = vec![];
        while let Some(res) = task_set.join_next().await {
            match res {
                Ok(Err(measure)) => {
                    failed.push(measure);
                }
                // In what case the await could failed ?
                _ => (),
            }
        }
        Ok(failed)
    }
}
