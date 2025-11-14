use crate::models::v1::profile_model::ProfileModel;
use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fs, sync::LazyLock};
use validator::{Validate, ValidationError};

static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9]+$").unwrap());

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProfileDTO {
    pub id: i32,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileDTO {
    #[validate(
        length(
            min = 3,
            max = 16,
            message = "Username must be between 3 and 16 characters"
        ),
        regex(
            path = *USERNAME_REGEX,
            message = "Username can only contain lowercase letters and numbers"
        )
    )]
    pub username: String,

    #[validate(length(
        min = 1,
        max = 32,
        message = "Display name must be between 1 and 32 characters"
    ))]
    pub display_name: Option<String>,

    #[validate(custom(function = "validate_profile_picture_size"))]
    #[serde(rename = "profilePictureBytes")]
    pub profile_picture_bytes: Option<Vec<u8>>,
}

impl TryFrom<ProfileModel> for GetProfileDTO {
    type Error = String;

    fn try_from(model: ProfileModel) -> Result<Self, Self::Error> {
        let avatar = model.profile_picture_url.as_ref().and_then(|path| {
            fs::read(path)
                .ok()
                .map(|bytes| general_purpose::STANDARD.encode(&bytes))
        });

        Ok(Self {
            id: model.id,
            username: model.username,
            display_name: model.display_name,
            avatar,
        })
    }
}

fn validate_profile_picture_size(bytes: &Vec<u8>) -> Result<(), ValidationError> {
    if bytes.len() > 2 * 1024 * 1024 {
        return Err(ValidationError::new("profile_picture_size")
            .with_message("Profile picture must be less than 2MB".into()));
    }

    Ok(())
}
