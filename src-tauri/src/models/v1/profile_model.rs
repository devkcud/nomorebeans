use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ProfileModel {
    pub id: i32,
    pub username: String,
    pub display_name: Option<String>,
    pub profile_picture_url: Option<String>,
}
