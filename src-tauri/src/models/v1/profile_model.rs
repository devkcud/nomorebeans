use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ProfileModel {
    pub id: i32,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,

    pub username: String,
    pub display_name: Option<String>,
    pub profile_picture_url: Option<String>,
}
