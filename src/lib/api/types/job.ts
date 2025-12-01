export type JobType = 'clt' | 'pj' | 'freelancer';

export type Job = {
    id: number;
    createdAt: Date;
    updatedAt: Date;
    companyName: string;
    positionTitle: string;
    salaryGross: number;
    jobType: JobType;
    dailyWorkHours: number;
    workdaysPerMonth: number;
};

export type CreateJobRequest = {
    companyName: string;
    positionTitle: string;
    salaryGross: number;
    jobType: JobType;
    dailyWorkHours: number;
    workdaysPerMonth: number;
};

export type UpdateJobRequest = {
    companyName?: string;
    positionTitle?: string;
    salaryGross?: number;
    jobType?: JobType;
    dailyWorkHours?: number;
    workdaysPerMonth?: number;
};

export const JOB_TYPE_LABELS: Record<JobType, string> = {
    clt: 'CLT',
    pj: 'PJ',
    freelancer: 'Freelancer'
};

export const JOB_TYPE_ICONS: Record<JobType, string> = {
    clt: 'mdi:badge-account-horizontal',
    pj: 'mdi:domain',
    freelancer: 'mdi:account-tie'
};
