use sqlx::PgPool;

use crate::{
    models::v1::profile_model,
    utils::{error::mapping::ErrorResponse, fs::profile_picture},
};

#[derive(Clone)]
pub struct ProfileRepository {
    pool: PgPool,
}

impl ProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_profile(
        &self,
        username: String,
        display_name: Option<String>,
        profile_picture_bytes: Option<Vec<u8>>,
    ) -> Result<profile_model::ProfileModel, ErrorResponse> {
        let profile_picture_url = if let Some(bytes) = profile_picture_bytes {
            let path = profile_picture::save_profile_picture(&bytes)?;
            Some(path.to_string_lossy().to_string())
        } else {
            None
        };

        let display_name = if let Some(display_name) = display_name {
            Some(display_name.trim().to_string())
        } else {
            None
        };

        let created_profile = sqlx::query_as::<_, profile_model::ProfileModel>(
            r#"
            INSERT INTO profiles (username, display_name, profile_picture_url)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(username)
        .bind(display_name)
        .bind(&profile_picture_url)
        .fetch_one(&self.pool)
        .await?;

        Ok(created_profile)
    }

    pub async fn get_all(&self) -> Result<Vec<profile_model::ProfileModel>, ErrorResponse> {
        let profiles = sqlx::query_as::<_, profile_model::ProfileModel>(
            r#"
            SELECT * FROM profiles WHERE deleted_at IS NULL
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(profiles)
    }
}
