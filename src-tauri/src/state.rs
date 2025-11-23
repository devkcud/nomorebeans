pub use crate::services;
use crate::{
    repositories::v1::{job_repository::JobRepository, profile_repository::ProfileRepository},
    services::{job_service::JobService, profile_service::ProfileService},
};

#[derive(Clone)]
pub struct AppState {
    pub profile_service: ProfileService,
    pub job_service: JobService,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool) -> Self {
        // Profile:
        let profile_repo = ProfileRepository::new(pool.clone());
        let profile_service = ProfileService::new(profile_repo);

        // Job:
        let job_repo = JobRepository::new(pool.clone());
        let job_service = JobService::new(job_repo);

        Self {
            profile_service,
            job_service,
        }
    }
}
