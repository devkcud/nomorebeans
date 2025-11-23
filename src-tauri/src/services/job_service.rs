use validator::Validate;

use crate::{
    repositories::v1::job_repository,
    services::dto::job_dto::{CreateJobDTO, GetJobDTO, UpdateJobDTO},
    utils::error::mapping::ErrorResponse,
};

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
                &new_job.try_into().map_err(|_| ErrorResponse::unhandled())?, // TODO: improve error handling
            )
            .await?;

        let dto = GetJobDTO::try_from(job).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }

    pub async fn get_all(&self, profile_id: i32) -> Result<Vec<GetJobDTO>, ErrorResponse> {
        let jobs = self.repo.get_all(profile_id).await;
        let dtos = jobs?
            .into_iter()
            .map(|job| GetJobDTO::try_from(job))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dtos)
    }

    pub async fn get_one_by_id(
        &self,
        profile_id: i32,
        job_id: i32,
    ) -> Result<GetJobDTO, ErrorResponse> {
        let job = self.repo.get_by_id(profile_id, job_id).await?;

        if let None = job {
            return Err(ErrorResponse::new(
                crate::utils::error::mapping::ErrorCode::SearchObjectNotFoundError,
                Some("id".into()),
                "Job not found",
            ));
        }

        let dto = GetJobDTO::try_from(job.unwrap()).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }

    pub async fn delete_one_by_id(
        &self,
        profile_id: i32,
        job_id: i32,
    ) -> Result<(), ErrorResponse> {
        self.repo.delete_job(profile_id, job_id).await
    }

    pub async fn update_one_by_id(
        &self,
        profile_id: i32,
        job_id: i32,
        updated_job: UpdateJobDTO,
    ) -> Result<GetJobDTO, ErrorResponse> {
        let job = self
            .repo
            .update(
                job_id,
                profile_id,
                updated_job
                    .try_into()
                    .map_err(|_| ErrorResponse::unhandled())?, // TODO: improve error handling
            )
            .await?;

        let dto = GetJobDTO::try_from(job).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }
}
