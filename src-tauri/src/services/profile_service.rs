use crate::{repositories, services::dto::profile_dto::GetProfileDTO};
use sqlx::Error;

#[derive(Clone)]
pub struct ProfileService {
    repo: repositories::v1::profile_repository::ProfileRepository,
}

impl ProfileService {
    pub fn new(repo: repositories::v1::profile_repository::ProfileRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<GetProfileDTO>, Error> {
        let users = self.repo.get_profiles().await?;
        Ok(users.into_iter().map(GetProfileDTO::from).collect())
    }
}
