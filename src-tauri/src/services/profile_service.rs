use crate::{repositories, services::domain::models::profile_domain_model::ProfileDomainModel};
use sqlx::Error;

#[derive(Clone)]
pub struct ProfileService {
    repo: repositories::v1::profile_repository::ProfileRepository,
}

impl ProfileService {
    pub fn new(repo: repositories::v1::profile_repository::ProfileRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<ProfileDomainModel>, Error> {
        let users = self.repo.get_profiles().await?;
        Ok(users.into_iter().map(ProfileDomainModel::from).collect())
    }
}
