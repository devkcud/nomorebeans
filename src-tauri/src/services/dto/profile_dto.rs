use crate::models::v1::profile_model::ProfileModel;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GetProfileDTO {
    pub id: i32,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl From<ProfileModel> for GetProfileDTO {
    fn from(model: ProfileModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            display_name: model.display_name,
            avatar_url: model.profile_picture_url,
        }
    }
}
