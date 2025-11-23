use crate::{models::v1::profile_model::ProfileModel, utils::validation::*};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProfileDTO {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileDTO {
    #[validate(
        length(min = USERNAME_MIN_LENGTH, max = USERNAME_MAX_LENGTH, message = "Username must be between 3 and 16 characters"),
        regex(path = *USERNAME_REGEX, message = "Username can only contain lowercase letters and numbers")
    )]
    pub username: String,

    #[validate(length(
        min = DISPLAY_NAME_MIN_LENGTH,
        max = DISPLAY_NAME_MAX_LENGTH,
        message = "Display name must be between 1 and 32 characters"
    ))]
    pub display_name: Option<String>,

    #[validate(custom(function = "validate_profile_picture_size"))]
    pub profile_picture_bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileDTO {
    #[validate(
        length(min = USERNAME_MIN_LENGTH, max = USERNAME_MAX_LENGTH, message = "Username must be between 3 and 16 characters"),
        regex(path = *USERNAME_REGEX, message = "Username can only contain lowercase letters and numbers")
    )]
    pub username: Option<String>,

    #[validate(length(
        min = DISPLAY_NAME_MIN_LENGTH,
        max = DISPLAY_NAME_MAX_LENGTH,
        message = "Display name must be between 1 and 32 characters"
    ))]
    pub display_name: Option<String>,

    #[validate(custom(function = "validate_profile_picture_size"))]
    pub profile_picture_bytes: Option<Vec<u8>>,
}

impl From<ProfileModel> for GetProfileDTO {
    fn from(model: ProfileModel) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            username: model.username,
            display_name: model.display_name,
            avatar: None,
        }
    }
}

impl GetProfileDTO {
    pub fn with_avatar(mut self, avatar: Option<String>) -> Self {
        self.avatar = avatar;
        self
    }
}

fn validate_profile_picture_size(bytes: &Vec<u8>) -> Result<(), ValidationError> {
    if bytes.len() > PROFILE_PICTURE_MAX_SIZE {
        return Err(ValidationError::new("profile_picture_size")
            .with_message(PROFILE_PICTURE_SIZE_MESSAGE.into()));
    }

    Ok(())
}
