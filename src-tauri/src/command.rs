use crate::{
    services::dto::profile_dto::{CreateProfileDTO, GetProfileDTO, UpdateProfileDTO},
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

#[tauri::command]
pub async fn get_profile_by_id(
    state: State<'_, AppState>,
    id: i32,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.get_one_by_id(id).await
}

#[tauri::command]
pub async fn get_profile_by_username(
    state: State<'_, AppState>,
    username: &str,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.get_one_by_username(username).await
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, AppState>, id: i32) -> Result<(), ErrorResponse> {
    state.profile_service.delete_profile(id).await
}

#[tauri::command]
pub async fn update_profile(
    state: State<'_, AppState>,
    id: i32,
    profile: UpdateProfileDTO,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.update_profile(id, profile).await
}
