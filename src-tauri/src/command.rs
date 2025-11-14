use crate::{
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO},
    state::AppState,
    utils::error::mapping::ErrorResponse,
};
use tauri::State;

#[tauri::command]
pub async fn get_profiles(state: State<'_, AppState>) -> Result<Vec<GetProfileDTO>, ErrorResponse> {
    state.profile_service.get_all().await
}

#[tauri::command]
pub async fn create_profile(
    state: State<'_, AppState>,
    profile: CreateProfileDTO,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.create_profile(profile).await
}
