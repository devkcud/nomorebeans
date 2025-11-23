CREATE TYPE job_type AS ENUM ('clt', 'pj', 'freelancer');

CREATE TABLE jobs (
    id SERIAL PRIMARY KEY,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),    
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP NULL,

    company_name TEXT NOT NULL,
    position_title TEXT NOT NULL,
    salary_gross INTEGER NOT NULL,

    job_type job_type NOT NULL,

    daily_work_hours INTEGER NOT NULL,
    workdays_per_month INTEGER NOT NULL,

    profile_owner_id INTEGER NOT NULL REFERENCES profiles(id)
);

CREATE INDEX idx_jobs_profile_owner_id ON jobs(profile_owner_id);
