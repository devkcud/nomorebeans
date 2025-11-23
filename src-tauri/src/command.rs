use crate::{
    services::dto::{
        job_dto::{CreateJobDTO, GetJobDTO, UpdateJobDTO},
        profile_dto::{CreateProfileDTO, GetProfileDTO, UpdateProfileDTO},
    },
    state::AppState,
    utils::error::mapping::ErrorResponse,
};
use tauri::State;

#[tauri::command]
pub async fn get_profiles(state: State<'_, AppState>) -> Result<Vec<GetProfileDTO>, ErrorResponse> {
    state.profile_service.fetch_all().await
}

#[tauri::command]
pub async fn create_profile(
    state: State<'_, AppState>,
    profile: CreateProfileDTO,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.create(profile).await
}

#[tauri::command]
pub async fn get_profile_by_id(
    state: State<'_, AppState>,
    id: i32,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.fetch_by_id(id).await
}

#[tauri::command]
pub async fn get_profile_by_username(
    state: State<'_, AppState>,
    username: &str,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.fetch_by_username(username).await
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, AppState>, id: i32) -> Result<(), ErrorResponse> {
    state.profile_service.delete(id).await
}

#[tauri::command]
pub async fn update_profile(
    state: State<'_, AppState>,
    id: i32,
    profile: UpdateProfileDTO,
) -> Result<GetProfileDTO, ErrorResponse> {
    state.profile_service.update(id, profile).await
}

#[tauri::command]
pub async fn create_job(
    state: State<'_, AppState>,
    profile_id: i32,
    job: CreateJobDTO,
) -> Result<GetJobDTO, ErrorResponse> {
    state.job_service.create(profile_id, job).await
}

#[tauri::command]
pub async fn get_jobs(
    state: State<'_, AppState>,
    profile_id: i32,
) -> Result<Vec<GetJobDTO>, ErrorResponse> {
    state.job_service.fetch_all_for_profile(profile_id).await
}

#[tauri::command]
pub async fn get_job_by_id(
    state: State<'_, AppState>,
    profile_id: i32,
    job_id: i32,
) -> Result<GetJobDTO, ErrorResponse> {
    state.job_service.fetch_by_id(profile_id, job_id).await
}

#[tauri::command]
pub async fn delete_job(
    state: State<'_, AppState>,
    profile_id: i32,
    job_id: i32,
) -> Result<(), ErrorResponse> {
    state.job_service.delete(profile_id, job_id).await
}

#[tauri::command]
pub async fn update_job(
    state: State<'_, AppState>,
    profile_id: i32,
    job_id: i32,
    job: UpdateJobDTO,
) -> Result<GetJobDTO, ErrorResponse> {
    state.job_service.update(profile_id, job_id, job).await
}
