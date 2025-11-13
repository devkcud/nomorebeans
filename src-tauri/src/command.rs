use crate::{
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO},
    state::AppState,
};
use tauri::State;

#[tauri::command]
pub async fn get_profiles(state: State<'_, AppState>) -> Result<Vec<GetProfileDTO>, String> {
    state
        .profile_service
        .get_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_profile(
    state: State<'_, AppState>,
    profile: CreateProfileDTO,
) -> Result<GetProfileDTO, String> {
    state
        .profile_service
        .create_profile(profile)
        .await
        .map_err(|e| e.to_string())
}
