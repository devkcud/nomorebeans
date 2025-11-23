use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::{FromRow, Type};

#[derive(Type, Debug)]
#[sqlx(type_name = "job_type", rename_all = "lowercase")]
pub enum JobType {
    CLT,
    PJ,
    Freelancer,
}

impl JobType {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobType::CLT => "clt",
            JobType::PJ => "pj",
            JobType::Freelancer => "freelancer",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "clt" => Ok(JobType::CLT),
            "pj" => Ok(JobType::PJ),
            "freelancer" => Ok(JobType::Freelancer),
            _ => Err(format!(
                "Invalid job type: '{}'. Must be one of: clt, pj, freelancer",
                s
            )),
        }
    }

    pub fn all_values() -> &'static [&'static str] {
        &["clt", "pj", "freelancer"]
    }
}

#[derive(FromRow, Debug)]
pub struct JobModel {
    pub id: i32,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,

    pub company_name: String,
    pub position_title: String,
    pub salary_gross: Decimal,

    pub job_type: JobType,

    pub daily_work_hours: i32,
    pub workdays_per_month: i32,

    pub profile_owner_id: i32,
}

#[derive(Debug)]
pub struct CreateJobModel {
    pub company_name: String,
    pub position_title: String,
    pub salary_gross: Decimal,
    pub job_type: JobType,
    pub daily_work_hours: i32,
    pub workdays_per_month: i32,
}

#[derive(Debug)]
pub struct UpdateJobModel {
    pub company_name: Option<String>,
    pub position_title: Option<String>,
    pub salary_gross: Option<Decimal>,
    pub job_type: Option<JobType>,
    pub daily_work_hours: Option<i32>,
    pub workdays_per_month: Option<i32>,
}
