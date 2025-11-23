use crate::{
    repositories::v1::job_repository,
    services::dto::job_dto::{CreateJobDTO, GetJobDTO, UpdateJobDTO},
    utils::error::mapping::ErrorResponse,
};
use validator::Validate;

#[derive(Clone)]
pub struct JobService {
    repo: job_repository::JobRepository,
}

impl JobService {
    pub fn new(repo: job_repository::JobRepository) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        profile_id: i32,
        new_job: CreateJobDTO,
    ) -> Result<GetJobDTO, ErrorResponse> {
        new_job.validate()?;

        let job = self
            .repo
            .create(
                profile_id,
                &new_job
                    .try_into()
                    .map_err(|e: String| ErrorResponse::validation_error("job_type", &e))?,
            )
            .await?;

        Ok(GetJobDTO::from(job))
    }

    pub async fn fetch_all_for_profile(
        &self,
        profile_id: i32,
    ) -> Result<Vec<GetJobDTO>, ErrorResponse> {
        let jobs = self.repo.fetch_all_for_profile(profile_id).await?;
        let dtos = jobs.into_iter().map(GetJobDTO::from).collect();

        Ok(dtos)
    }

    pub async fn fetch_by_id(
        &self,
        profile_id: i32,
        job_id: i32,
    ) -> Result<GetJobDTO, ErrorResponse> {
        let job = self.repo.fetch_by_id(profile_id, job_id).await?;

        job.map(GetJobDTO::from)
            .ok_or(ErrorResponse::object_not_found("id", "Job not found"))
    }

    pub async fn update(
        &self,
        profile_id: i32,
        job_id: i32,
        updated_job: UpdateJobDTO,
    ) -> Result<GetJobDTO, ErrorResponse> {
        updated_job.validate()?;

        let job = self
            .repo
            .update(
                profile_id,
                job_id,
                updated_job
                    .try_into()
                    .map_err(|e: String| ErrorResponse::validation_error("job_type", &e))?,
            )
            .await?;

        Ok(GetJobDTO::from(job))
    }

    pub async fn delete(&self, profile_id: i32, job_id: i32) -> Result<(), ErrorResponse> {
        self.repo.delete(profile_id, job_id).await
    }
}
