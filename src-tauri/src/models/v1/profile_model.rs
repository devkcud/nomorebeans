use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ProfileModel {
    pub id: i32,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub username: String,
    pub display_name: Option<String>,
    pub profile_picture_url: Option<String>,
}
