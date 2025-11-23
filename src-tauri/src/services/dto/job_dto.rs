use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    models::v1::job_model::{self, JobModel, NewJobModel, UpdateJobModel},
    utils::math::constants::ARITHMETIC_SCALE,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryGrossDTO {
    pub salary_gross: i64,
    pub arithmetic_scale: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetJobDTO {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub company_name: String,
    pub position_title: String,
    pub salary_gross: SalaryGrossDTO,
    pub job_type: String,
    pub daily_work_hours: i32,
    pub workdays_per_month: i32,
    pub profile_owner_id: i32,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobDTO {
    #[validate(length(
        min = 1,
        max = 64,
        message = "Company name must be between 1 and 64 characters"
    ))]
    pub company_name: String,
    #[validate(length(
        min = 1,
        max = 64,
        message = "Position title must be between 1 and 64 characters"
    ))]
    pub position_title: String,
    #[validate(range(min = 0, message = "Salary gross must be a non-negative value"))]
    pub salary_gross: i64,
    #[validate(custom(function = "validate_job_type"))]
    pub job_type: String,
    #[validate(range(
        min = 1,
        max = 24,
        message = "Daily work hours must be between 1 and 24"
    ))]
    pub daily_work_hours: i32,
    #[validate(range(
        min = 1,
        max = 31,
        message = "Workdays per month must be between 1 and 31"
    ))]
    pub workdays_per_month: i32,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateJobDTO {
    #[validate(length(
        min = 1,
        max = 64,
        message = "Company name must be between 1 and 64 characters"
    ))]
    pub company_name: Option<String>,
    #[validate(length(
        min = 1,
        max = 64,
        message = "Position title must be between 1 and 64 characters"
    ))]
    pub position_title: Option<String>,
    #[validate(range(min = 0, message = "Salary gross must be a non-negative value"))]
    pub salary_gross: Option<i64>,
    #[validate(custom(function = "validate_job_type"))]
    pub job_type: Option<String>,
    #[validate(range(
        min = 1,
        max = 24,
        message = "Daily work hours must be between 1 and 24"
    ))]
    pub daily_work_hours: Option<i32>,
    #[validate(range(
        min = 1,
        max = 31,
        message = "Workdays per month must be between 1 and 31"
    ))]
    pub workdays_per_month: Option<i32>,
}

impl TryFrom<JobModel> for GetJobDTO {
    type Error = String;

    fn try_from(job_model: JobModel) -> Result<Self, Self::Error> {
        let job_type_str = match job_model.job_type {
            job_model::JobType::CLT => "clt",
            job_model::JobType::PJ => "pj",
            job_model::JobType::Freelance => "freelance",
        }
        .to_string();

        Ok(GetJobDTO {
            id: job_model.id,
            created_at: job_model.created_at,
            updated_at: job_model.updated_at,
            company_name: job_model.company_name,
            position_title: job_model.position_title,
            salary_gross: SalaryGrossDTO {
                salary_gross: job_model.salary_gross,
                arithmetic_scale: ARITHMETIC_SCALE,
            },
            job_type: job_type_str,
            daily_work_hours: job_model.daily_work_hours,
            workdays_per_month: job_model.workdays_per_month,
            profile_owner_id: job_model.profile_owner_id,
        })
    }
}

impl TryInto<NewJobModel> for CreateJobDTO {
    type Error = String;

    fn try_into(self) -> Result<NewJobModel, Self::Error> {
        let job_type_enum = match self.job_type.to_lowercase().as_str() {
            "clt" => job_model::JobType::CLT,
            "pj" => job_model::JobType::PJ,
            "freelance" => job_model::JobType::Freelance,
            _ => return Err("Invalid job type".into()),
        };

        Ok(NewJobModel {
            company_name: self.company_name,
            position_title: self.position_title,
            salary_gross: self.salary_gross,
            job_type: job_type_enum,
            daily_work_hours: self.daily_work_hours,
            workdays_per_month: self.workdays_per_month,
            profile_owner_id: 0, // This should be set in the service layer
        })
    }
}

impl TryInto<UpdateJobModel> for UpdateJobDTO {
    type Error = String;

    fn try_into(self) -> Result<UpdateJobModel, Self::Error> {
        let job_type_enum = match self.job_type {
            Some(ref jt) => match jt.to_lowercase().as_str() {
                "clt" => Some(job_model::JobType::CLT),
                "pj" => Some(job_model::JobType::PJ),
                "freelance" => Some(job_model::JobType::Freelance),
                _ => return Err("Invalid job type".into()),
            },
            None => None,
        };

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
    let valid_types = ["clt", "pj", "freelance"];
    if valid_types.contains(&job_type.to_lowercase().as_str()) {
        Ok(())
    } else {
        Err(ValidationError::new(
            "Invalid job type. Must be one of: clt, pj, freelance",
        ))
    }
}
