use crate::{services::domain::models::profile_domain_model::ProfileDomainModel, state::AppState};
use tauri::State;

#[tauri::command]
pub async fn get_profiles(state: State<'_, AppState>) -> Result<Vec<ProfileDomainModel>, String> {
    state
        .profile_service
        .get_all()
        .await
        .map_err(|e| e.to_string())
}
