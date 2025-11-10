use sqlx::PgPool;

use crate::models::v1::profile_model;

#[derive(Clone)]
pub struct ProfileRepository {
    pool: PgPool,
}

impl ProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_profiles(&self) -> Result<Vec<profile_model::ProfileModel>, sqlx::Error> {
        let profiles = sqlx::query_as::<_, profile_model::ProfileModel>("SELECT * FROM profiles")
            .fetch_all(&self.pool)
            .await?;
        Ok(profiles)
    }
}
