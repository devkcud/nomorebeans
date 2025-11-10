pub use crate::services;
use crate::{repositories::v1::profile_repository::ProfileRepository, services::profile_service::ProfileService};

#[derive(Clone)]
pub struct AppState {
    pub profile_service: ProfileService,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool) -> Self {
        // Profile:
        let profile_repo = ProfileRepository::new(pool.clone());
        let profile_service = ProfileService::new(profile_repo);

        Self { profile_service }
    }
}
