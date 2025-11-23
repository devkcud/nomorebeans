use sqlx::PgPool;

use crate::{models::v1::job_model, utils::error::mapping::ErrorResponse};

#[derive(Clone)]
pub struct JobRepository {
    pool: PgPool,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        profile_id: i32,
        new_job: &job_model::NewJobModel,
    ) -> Result<job_model::JobModel, ErrorResponse> {
        let created_job = sqlx::query_as::<_, job_model::JobModel>(
            r#"
            INSERT INTO jobs (company_name, position_title, salary_gross, job_type, daily_work_hours, workdays_per_month, profile_owner_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(&new_job.company_name)
        .bind(&new_job.position_title)
        .bind(new_job.salary_gross)
        .bind(&new_job.job_type)
        .bind(new_job.daily_work_hours)
        .bind(new_job.workdays_per_month)
        .bind(profile_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(created_job)
    }

    pub async fn get_all(
        &self,
        profile_id: i32,
    ) -> Result<Vec<job_model::JobModel>, ErrorResponse> {
        let jobs = sqlx::query_as::<_, job_model::JobModel>(
            r#"
            SELECT * FROM jobs
            WHERE profile_owner_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(profile_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }

    pub async fn get_by_id(
        &self,
        profile_id: i32,
        job_id: i32,
    ) -> Result<Option<job_model::JobModel>, ErrorResponse> {
        let job = sqlx::query_as::<_, job_model::JobModel>(
            r#"
            SELECT * FROM jobs
            WHERE id = $1 AND profile_owner_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(job_id)
        .bind(profile_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn update(
        &self,
        job_id: i32,
        profile_id: i32,
        updated_job: job_model::UpdateJobModel,
    ) -> Result<job_model::JobModel, ErrorResponse> {
        let updated_job = sqlx::query_as::<_, job_model::JobModel>(
            r#"
            UPDATE jobs SET
                company_name = COALESCE($1, company_name),
                position_title = COALESCE($2, position_title),
                salary_gross = COALESCE($3, salary_gross),
                job_type = COALESCE($4, job_type),
                daily_work_hours = COALESCE($5, daily_work_hours),
                workdays_per_month = COALESCE($6, workdays_per_month),
                updated_at = NOW()
            WHERE id = $7 AND profile_owner_id = $8 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(&updated_job.company_name)
        .bind(&updated_job.position_title)
        .bind(updated_job.salary_gross)
        .bind(&updated_job.job_type)
        .bind(updated_job.daily_work_hours)
        .bind(updated_job.workdays_per_month)
        .bind(job_id)
        .bind(profile_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_job)
    }

    pub async fn delete_job(&self, job_id: i32, profile_id: i32) -> Result<(), ErrorResponse> {
        sqlx::query(
            r#"
            UPDATE jobs SET deleted_at = NOW()
            WHERE id = $1 AND profile_owner_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(job_id)
        .bind(profile_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
