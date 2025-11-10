use crate::{services::dto::profile_dto::GetProfileDTO, state::AppState};
use tauri::State;

#[tauri::command]
pub async fn get_profiles(state: State<'_, AppState>) -> Result<Vec<GetProfileDTO>, String> {
    state
        .profile_service
        .get_all()
        .await
        .map_err(|e| e.to_string())
}
