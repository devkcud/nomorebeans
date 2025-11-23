use crate::{
    models::v1::profile_model::ProfileModel,
    repositories::v1::profile_repository,
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO, UpdateProfileDTO},
    utils::error::mapping::ErrorResponse,
};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use validator::Validate;

#[derive(Clone)]
pub struct ProfileService {
    repo: profile_repository::ProfileRepository,
}

impl ProfileService {
    pub fn new(repo: profile_repository::ProfileRepository) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        new_profile: CreateProfileDTO,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        new_profile.validate()?;

        let profile = self
            .repo
            .create(
                new_profile.username,
                new_profile.display_name,
                new_profile.profile_picture_bytes,
            )
            .await?;

        let dto = map_profile(profile)?;

        Ok(dto)
    }

    pub async fn fetch_all(&self) -> Result<Vec<GetProfileDTO>, ErrorResponse> {
        let users: Vec<ProfileModel> = self.repo.fetch_all().await?;
        let dtos: Vec<GetProfileDTO> = users
            .into_iter()
            .map(|model| {
                let avatar = load_avatar(&model.profile_picture_url);
                GetProfileDTO::from(model).with_avatar(avatar)
            })
            .collect();

        Ok(dtos)
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

    pub async fn update(
        &self,
        id: i32,
        updates: UpdateProfileDTO,
    ) -> Result<GetProfileDTO, ErrorResponse> {
        updates.validate()?;

        let profile = self
            .repo
            .update(
                id,
                updates.username,
                updates.display_name,
                updates.profile_picture_bytes,
            )
            .await?;

        let dto = map_profile(profile)?;

        Ok(dto)
    }

    pub async fn delete(&self, id: i32) -> Result<(), ErrorResponse> {
        self.repo.delete(id).await
    }
}

fn map_profile(model: ProfileModel) -> Result<GetProfileDTO, ErrorResponse> {
    let avatar = load_avatar(&model.profile_picture_url);
    Ok(GetProfileDTO::from(model).with_avatar(avatar))
}

fn load_avatar(path: &Option<String>) -> Option<String> {
    path.as_ref().and_then(|p| {
        fs::read(p)
            .ok()
            .map(|bytes| general_purpose::STANDARD.encode(&bytes))
    })
}
