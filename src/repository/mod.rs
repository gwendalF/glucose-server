use sqlx::{Pool, Postgres};

use crate::{
    config::AppConfig,
    entities::{errors::AppResult, DatabaseMeasure, GetDataQuery, GlucoseMeasure},
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
        user: i64,
        measures: Vec<GlucoseMeasure>,
    ) -> AppResult<Vec<GlucoseMeasure>> {
        let futures_insert: Vec<_> = measures
            .into_iter()
            .map(|measure| async {
                match sqlx::query!(
                    r#"
                INSERT INTO cgm_values (time, raw_type, date, sgv, direction, noise, user_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
                    measure.date,
                    measure.raw_type,
                    measure.timestamp as i64,
                    measure.sgv as i32,
                    measure.direction,
                    measure.noise as i32,
                    user
                )
                .execute(&self.database)
                .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(measure),
                }
            })
            .collect();
        let failed = futures::future::join_all(futures_insert)
            .await
            .into_iter()
            .filter_map(|db_result| match db_result {
                Ok(_) => None,
                Err(measure) => Some(measure),
            })
            .collect();
        Ok(failed)
    }

    pub async fn get_measures(
        &self,
        date: GetDataQuery,
        user_id: i64,
    ) -> AppResult<Vec<GlucoseMeasure>> {
        let data = sqlx::query_as!(
            DatabaseMeasure,
            r#"
            SELECT time as date, raw_type, date as timestamp, sgv, direction, noise 
            FROM cgm_values 
            WHERE cgm_values.user_id = $1
            AND  time > $2
            AND time < $3
        "#,
            user_id,
            date.from,
            date.to
        )
        .fetch_all(&self.database)
        .await?;
        let domain_data = data
            .into_iter()
            .filter_map(|d| match d.try_into() {
                Ok(domain) => Some(domain),
                Err(_) => None,
            })
            .collect();
        Ok(domain_data)
    }
}
