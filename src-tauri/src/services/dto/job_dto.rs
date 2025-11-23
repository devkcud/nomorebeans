use crate::{
    models::v1::job_model::{CreateJobModel, JobModel, JobType, UpdateJobModel},
    utils::validation::*,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetJobDTO {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub company_name: String,
    pub position_title: String,
    pub salary_gross: Decimal,
    pub job_type: String,
    pub daily_work_hours: i32,
    pub workdays_per_month: i32,
    pub profile_owner_id: i32,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobDTO {
    #[validate(length(
        min = COMPANY_NAME_MIN_LENGTH,
        max = COMPANY_NAME_MAX_LENGTH,
        message = "Company name must be between 1 and 64 characters"
    ))]
    pub company_name: String,
    #[validate(length(
        min = POSITION_TITLE_MIN_LENGTH,
        max = POSITION_TITLE_MAX_LENGTH,
        message = "Position title must be between 1 and 64 characters"
    ))]
    pub position_title: String,
    #[validate(custom(function = "validate_salary_gross"))]
    pub salary_gross: Decimal,
    #[validate(custom(function = "validate_job_type"))]
    pub job_type: String,
    #[validate(range(
        min = DAILY_WORK_HOURS_MIN,
        max = DAILY_WORK_HOURS_MAX,
        message = "Daily work hours must be between 1 and 24"
    ))]
    pub daily_work_hours: i32,
    #[validate(range(
        min = WORKDAYS_PER_MONTH_MIN,
        max = WORKDAYS_PER_MONTH_MAX,
        message = "Workdays per month must be between 1 and 30"
    ))]
    pub workdays_per_month: i32,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateJobDTO {
    #[validate(length(
        min = COMPANY_NAME_MIN_LENGTH,
        max = COMPANY_NAME_MAX_LENGTH,
        message = "Company name must be between 1 and 64 characters"
    ))]
    pub company_name: Option<String>,
    #[validate(length(
        min = POSITION_TITLE_MIN_LENGTH,
        max = POSITION_TITLE_MAX_LENGTH,
        message = "Position title must be between 1 and 64 characters"
    ))]
    pub position_title: Option<String>,
    #[validate(custom(function = "validate_salary_gross"))]
    pub salary_gross: Option<Decimal>,
    #[validate(custom(function = "validate_job_type"))]
    pub job_type: Option<String>,
    #[validate(range(
        min = DAILY_WORK_HOURS_MIN,
        max = DAILY_WORK_HOURS_MAX,
        message = "Daily work hours must be between 1 and 24"
    ))]
    pub daily_work_hours: Option<i32>,
    #[validate(range(
        min = WORKDAYS_PER_MONTH_MIN,
        max = WORKDAYS_PER_MONTH_MAX,
        message = "Workdays per month must be between 1 and 30"
    ))]
    pub workdays_per_month: Option<i32>,
}

impl From<JobModel> for GetJobDTO {
    fn from(job_model: JobModel) -> Self {
        GetJobDTO {
            id: job_model.id,
            created_at: job_model.created_at,
            updated_at: job_model.updated_at,
            company_name: job_model.company_name,
            position_title: job_model.position_title,
            salary_gross: job_model.salary_gross,
            job_type: job_model.job_type.as_str().to_string(),
            daily_work_hours: job_model.daily_work_hours,
            workdays_per_month: job_model.workdays_per_month,
            profile_owner_id: job_model.profile_owner_id,
        }
    }
}

impl TryInto<CreateJobModel> for CreateJobDTO {
    type Error = String;

    fn try_into(self) -> Result<CreateJobModel, Self::Error> {
        let job_type_enum = JobType::from_str(&self.job_type)?;

        Ok(CreateJobModel {
            company_name: self.company_name,
            position_title: self.position_title,
            salary_gross: self.salary_gross,
            job_type: job_type_enum,
            daily_work_hours: self.daily_work_hours,
            workdays_per_month: self.workdays_per_month,
        })
    }
}

impl TryInto<UpdateJobModel> for UpdateJobDTO {
    type Error = String;

    fn try_into(self) -> Result<UpdateJobModel, Self::Error> {
        let job_type_enum = self.job_type.map(|jt| JobType::from_str(&jt)).transpose()?;

        Ok(UpdateJobModel {
            company_name: self.company_name,
            position_title: self.position_title,
            salary_gross: self.salary_gross,
            job_type: job_type_enum,
            daily_work_hours: self.daily_work_hours,
            workdays_per_month: self.workdays_per_month,
        })
    }
}

fn validate_job_type(job_type: &str) -> Result<(), ValidationError> {
    if JobType::all_values().contains(&job_type.to_lowercase().as_str()) {
        Ok(())
    } else {
        Err(ValidationError::new(
            "Invalid job type. Must be one of: clt, pj, freelancer",
        ))
    }
}

fn validate_salary_gross(salary: &Decimal) -> Result<(), ValidationError> {
    if salary.is_sign_negative() {
        Err(ValidationError::new(SALARY_GROSS_NEGATIVE_MESSAGE))
    } else {
        Ok(())
    }
}
