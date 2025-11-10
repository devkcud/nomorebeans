use crate::models::v1::profile_model::ProfileModel;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProfileDomainModel {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl From<ProfileModel> for ProfileDomainModel {
    fn from(model: ProfileModel) -> Self {
        Self {
            username: model.username,
            display_name: model.display_name,
            avatar_url: model.profile_picture_url,
        }
    }
}
