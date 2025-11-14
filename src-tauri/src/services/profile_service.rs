use crate::{
    models::v1::profile_model::ProfileModel,
    repositories,
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO},
    utils::error::mapping::ErrorResponse,
};
use validator::Validate;

#[derive(Clone)]
pub struct ProfileService {
    repo: repositories::v1::profile_repository::ProfileRepository,
}

impl ProfileService {
    pub fn new(repo: repositories::v1::profile_repository::ProfileRepository) -> Self {
        Self { repo }
    }

    pub async fn create_profile(
        &self,
        profile: CreateProfileDTO,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        profile.validate()?;

        let profile = self
            .repo
            .create_profile(
                profile.username,
                profile.display_name,
                profile.profile_picture_bytes,
            )
            .await?;

        let dto = GetProfileDTO::try_from(profile).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }

    pub async fn get_all(&self) -> Result<Vec<GetProfileDTO>, ErrorResponse> {
        let users: Vec<ProfileModel> = self.repo.get_profiles().await?;
        let dtos: Result<Vec<_>, _> = users.into_iter().map(GetProfileDTO::try_from).collect();

        dtos.map_err(|_| ErrorResponse::unhandled()) // TODO: improve error handling
    }
}
