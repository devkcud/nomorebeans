use crate::{
    models::v1::profile_model::ProfileModel,
    repositories,
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO, UpdateProfileDTO},
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

    pub async fn create(&self, profile: CreateProfileDTO) -> Result<GetProfileDTO, ErrorResponse> {
        profile.validate()?;

        let profile = self
            .repo
            .create(
                profile.username,
                profile.display_name,
                profile.profile_picture_bytes,
            )
            .await?;

        let dto = map_profile(profile)?;

        Ok(dto)
    }

    pub async fn fetch_all(&self) -> Result<Vec<GetProfileDTO>, ErrorResponse> {
        let users: Vec<ProfileModel> = self.repo.fetch_all().await?;
        let dtos: Result<Vec<_>, _> = users.into_iter().map(GetProfileDTO::try_from).collect();

        dtos.map_err(|_| ErrorResponse::unhandled())
    }

    pub async fn fetch_by_id(&self, id: i32) -> Result<GetProfileDTO, ErrorResponse> {
        let profile: Option<ProfileModel> = self.repo.fetch_by_id(id).await?;

        profile
            .map(map_profile)
            .transpose()?
            .ok_or(ErrorResponse::object_not_found("id", "Profile not found"))
    }

    pub async fn fetch_by_username(
        &self,
        username: impl Into<String>,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        let profile: Option<ProfileModel> = self.repo.fetch_by_username(username.into()).await?;

        profile
            .map(map_profile)
            .transpose()?
            .ok_or(ErrorResponse::object_not_found(
                "username",
                "Profile not found",
            ))
    }

    pub async fn delete(&self, id: i32) -> Result<(), ErrorResponse> {
        self.repo.delete(id).await
    }

    pub async fn update(
        &self,
        id: i32,
        profile: UpdateProfileDTO,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        profile.validate()?;

        let profile = self
            .repo
            .update(
                id,
                profile.username,
                profile.display_name,
                profile.profile_picture_bytes,
            )
            .await?;

        let dto = map_profile(profile)?;

        Ok(dto)
    }
}

fn map_profile(model: ProfileModel) -> Result<GetProfileDTO, ErrorResponse> {
    GetProfileDTO::try_from(model).map_err(|_| ErrorResponse::unhandled())
}
