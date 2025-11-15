use crate::{
    models::v1::profile_model::ProfileModel,
    repositories,
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO, UpdateProfileDTO},
    utils::error::mapping::{ErrorCode, ErrorResponse},
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
        let users: Vec<ProfileModel> = self.repo.get_all().await?;
        let dtos: Result<Vec<_>, _> = users.into_iter().map(GetProfileDTO::try_from).collect();

        dtos.map_err(|_| ErrorResponse::unhandled()) // TODO: improve error handling
    }

    pub async fn get_one_by_id(&self, id: i32) -> Result<GetProfileDTO, ErrorResponse> {
        let profile: Option<ProfileModel> = self.repo.get_one_by_id(id).await?;

        if let None = profile {
            return Err(ErrorResponse::new(
                ErrorCode::SearchObjectNotFoundError,
                Some("id".into()),
                "Profile not found",
            ));
        }

        let dto =
            GetProfileDTO::try_from(profile.unwrap()).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }

    pub async fn get_one_by_username(
        &self,
        username: impl Into<String>,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        let profile: Option<ProfileModel> = self.repo.get_one_by_username(username.into()).await?;

        if let None = profile {
            return Err(ErrorResponse::new(
                ErrorCode::SearchObjectNotFoundError,
                Some("username".into()),
                "Profile not found",
            ));
        }

        let dto =
            GetProfileDTO::try_from(profile.unwrap()).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }

    pub async fn delete_profile(&self, id: i32) -> Result<(), ErrorResponse> {
        self.repo.delete_profile(id).await
    }

    pub async fn update_profile(
        &self,
        id: i32,
        profile: UpdateProfileDTO,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        let profile = self
            .repo
            .update_profile(
                id,
                profile.username,
                profile.display_name,
                profile.profile_picture_bytes,
            )
            .await?;

        let dto = GetProfileDTO::try_from(profile).map_err(|_| ErrorResponse::unhandled())?; // TODO: improve error handling

        Ok(dto)
    }
}
