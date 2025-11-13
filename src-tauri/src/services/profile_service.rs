use crate::{
    repositories,
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO},
};
use sqlx::Error;
use validator::Validate;

#[derive(Clone)]
pub struct ProfileService {
    repo: repositories::v1::profile_repository::ProfileRepository,
}

impl ProfileService {
    pub fn new(repo: repositories::v1::profile_repository::ProfileRepository) -> Self {
        Self { repo }
    }

    pub async fn create_profile(&self, profile: CreateProfileDTO) -> Result<GetProfileDTO, Error> {
        profile
            .validate()
            .map_err(|e| Error::Protocol(e.to_string().into()))?;

        let profile = self
            .repo
            .create_profile(
                profile.username,
                profile.display_name,
                profile.profile_picture_bytes,
            )
            .await?;

        let dto = GetProfileDTO::try_from(profile).map_err(|e| Error::Protocol(e.into()))?;

        Ok(dto)
    }

    pub async fn get_all(&self) -> Result<Vec<GetProfileDTO>, Error> {
        let users = self.repo.get_profiles().await?;
        let dtos: Result<Vec<_>, _> = users.into_iter().map(GetProfileDTO::try_from).collect();

        dtos.map_err(|e| Error::Protocol(e.into()))
    }
}
