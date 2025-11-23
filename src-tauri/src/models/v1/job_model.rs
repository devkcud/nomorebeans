use chrono::NaiveDateTime;
use sqlx::{FromRow, Type};

#[derive(Type, Debug)]
#[sqlx(type_name = "job_type", rename_all = "lowercase")]
pub enum JobType {
    CLT,
    PJ,
    Freelance,
}

#[derive(FromRow, Debug)]
pub struct JobModel {
    pub id: i32,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,

    pub company_name: String,
    pub position_title: String,
    pub salary_gross: i64,

    pub job_type: JobType,

    pub daily_work_hours: i32,
    pub workdays_per_month: i32,

    pub profile_owner_id: i32,
}

#[derive(Debug)]
pub struct NewJobModel {
    pub company_name: String,
    pub position_title: String,
    pub salary_gross: i64,
    pub job_type: JobType,
    pub daily_work_hours: i32,
    pub workdays_per_month: i32,
    pub profile_owner_id: i32,
}

#[derive(Debug)]
pub struct UpdateJobModel {
    pub company_name: Option<String>,
    pub position_title: Option<String>,
    pub salary_gross: Option<i64>,
    pub job_type: Option<JobType>,
    pub daily_work_hours: Option<i32>,
    pub workdays_per_month: Option<i32>,
}
