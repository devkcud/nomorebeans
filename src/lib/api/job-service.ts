import { invoke } from '@tauri-apps/api/core';
import type { Job, CreateJobRequest, UpdateJobRequest } from './types/job';
import type { ErrorResponse } from './types/error';

export async function getJobs(profileId: number): Promise<Job[]> {
    return await invoke<Job[]>('get_jobs', { profileId });
}

export async function createJob(profileId: number, job: CreateJobRequest): Promise<Job> {
    try {
        return await invoke<Job>('create_job', {
            profileId,
            job: {
                companyName: job.companyName,
                positionTitle: job.positionTitle,
                salaryGross: job.salaryGross,
                jobType: job.jobType,
                dailyWorkHours: job.dailyWorkHours,
                workdaysPerMonth: job.workdaysPerMonth
            }
        });
    } catch (err) {
        console.error(err);
        throw err as ErrorResponse;
    }
}

export async function updateJob(profileId: number, jobId: number, job: UpdateJobRequest): Promise<Job> {
    try {
        return await invoke<Job>('update_job', {
            profileId,
            jobId,
            job: {
                companyName: job.companyName,
                positionTitle: job.positionTitle,
                salaryGross: job.salaryGross,
                jobType: job.jobType,
                dailyWorkHours: job.dailyWorkHours,
                workdaysPerMonth: job.workdaysPerMonth
            }
        });
    } catch (err) {
        console.error(err);
        throw err as ErrorResponse;
    }
}

export async function deleteJob(profileId: number, jobId: number): Promise<void> {
    try {
        await invoke('delete_job', { profileId, jobId });
    } catch (err) {
        console.error(err);
        throw err as ErrorResponse;
    }
}
