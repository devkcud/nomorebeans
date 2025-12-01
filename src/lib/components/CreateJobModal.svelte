<script lang="ts">
    import { createJob } from '$lib/api/job-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { CreateJobRequest, JobType } from '$lib/api/types/job';
    import { JOB_TYPE_LABELS } from '$lib/api/types/job';
    import {
        MAX_COMPANY_NAME_LENGTH,
        MAX_POSITION_TITLE_LENGTH,
        MAX_DAILY_WORK_HOURS,
        MAX_WORK_DAYS_PER_MONTH,
        MODAL_CLOSE_ANIMATION_MS
    } from '$lib/constants';
    import { fade } from 'svelte/transition';

    interface Props {
        profileId: number;
        onSuccess?: () => void;
    }

    let { profileId, onSuccess }: Props = $props();

    let formData = $state<CreateJobRequest>({
        companyName: '',
        positionTitle: '',
        salaryGross: 0,
        jobType: 'clt',
        dailyWorkHours: 8,
        workdaysPerMonth: 20
    });

    let isLoading = $state(false);
    let error = $state<ErrorResponse | undefined>(undefined);

    let modalElement: HTMLDialogElement;
    let formElement: HTMLFormElement;

    async function handleSubmit(e: Event) {
        e.preventDefault();
        isLoading = true;

        try {
            await createJob(profileId, formData);
            modalElement.close();
            await resetForm();
            onSuccess?.();
        } catch (err) {
            error = err as ErrorResponse;
        } finally {
            isLoading = false;
        }
    }

    async function resetForm() {
        await new Promise((resolve) => setTimeout(resolve, MODAL_CLOSE_ANIMATION_MS));

        error = undefined;
        formData = {
            companyName: '',
            positionTitle: '',
            salaryGross: 0,
            jobType: 'clt',
            dailyWorkHours: 8,
            workdaysPerMonth: 20
        };
        formElement?.reset();
    }

    function clearError() {
        error = undefined;
    }

    export function open() {
        modalElement?.showModal();
    }

    const jobTypes: JobType[] = ['clt', 'pj', 'freelancer'];

    const positionSuggestions = [
        'Software Engineer',
        'Product Manager',
        'Designer',
        'Data Analyst',
        'DevOps Engineer',
        'QA Engineer',
        'Tech Lead',
        'Consultant'
    ];

    let monthlyHours = $derived(formData.dailyWorkHours * formData.workdaysPerMonth);

    function formatCurrency(value: number): string {
        return new Intl.NumberFormat('pt-BR', {
            style: 'currency',
            currency: 'BRL'
        }).format(value);
    }

    let hourlyRate = $derived(monthlyHours > 0 ? formData.salaryGross / monthlyHours : 0);
</script>

{#snippet maxLengthIndicator(value: string, max: number)}
    <span class="label" class:text-error={value.length > max}>
        {value.length}/{max}
    </span>
{/snippet}

<dialog class="modal backdrop-blur-sm" bind:this={modalElement} onclose={resetForm}>
    <div class="glass-effect modal-box w-full max-w-xl max-h-[90vh] overflow-y-auto border border-primary/20">
        <div
            class="pointer-events-none absolute inset-0 rounded-2xl bg-linear-to-br from-primary/5 via-transparent to-secondary/5"
        ></div>

        <form method="dialog">
            <button
                class="transition-smooth btn absolute top-2 right-2 z-10 btn-circle btn-ghost btn-sm hover:rotate-90 hover:bg-error/20"
                aria-label="Close modal"
            >
                <iconify-icon icon="mdi:close"></iconify-icon>
            </button>
        </form>

        <form class="relative z-10 space-y-4" bind:this={formElement} onsubmit={handleSubmit}>
            <h3
                class="bg-linear-to-r from-primary to-secondary bg-clip-text text-2xl font-bold text-transparent"
            >
                Add New Job
            </h3>

            <p class="pb-4 text-xs italic">
                <span class="text-error">*</span> required fields
            </p>

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">
                    Company Name <span class="text-error">*</span>
                </legend>
                <label
                    class="transition-smooth input w-full"
                    class:input-error={error?.field === 'company_name'}
                >
                    <iconify-icon icon="mdi:domain"></iconify-icon>
                    <input
                        type="text"
                        class="grow"
                        placeholder="Acme Corp"
                        bind:value={formData.companyName}
                        maxlength={MAX_COMPANY_NAME_LENGTH}
                        required
                    />
                </label>
                {@render maxLengthIndicator(formData.companyName, MAX_COMPANY_NAME_LENGTH)}
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">
                    Position Title <span class="text-error">*</span>
                </legend>
                <label
                    class="transition-smooth input w-full"
                    class:input-error={error?.field === 'position_title'}
                >
                    <iconify-icon icon="mdi:briefcase"></iconify-icon>
                    <input
                        type="text"
                        class="grow"
                        placeholder="Software Engineer"
                        bind:value={formData.positionTitle}
                        maxlength={MAX_POSITION_TITLE_LENGTH}
                        required
                    />
                </label>
                <div class="mt-2 flex flex-wrap gap-1">
                    {#each positionSuggestions as suggestion}
                        <button
                            type="button"
                            class="transition-smooth rounded-full border border-white/10 px-2 py-0.5 text-xs text-base-content/60 hover:border-primary/50 hover:bg-primary/10 hover:text-primary"
                            onclick={() => (formData.positionTitle = suggestion)}
                        >
                            {suggestion}
                        </button>
                    {/each}
                </div>
                {@render maxLengthIndicator(formData.positionTitle, MAX_POSITION_TITLE_LENGTH)}
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">
                    Gross Salary <span class="text-error">*</span>
                </legend>
                <label
                    class="transition-smooth input w-full"
                    class:input-error={error?.field === 'salary_gross'}
                >
                    <iconify-icon icon="mdi:currency-usd"></iconify-icon>
                    <input
                        type="number"
                        class="grow"
                        placeholder="5000"
                        bind:value={formData.salaryGross}
                        min={0}
                        step="0.01"
                        required
                    />
                </label>
                <span class="label">Monthly gross salary</span>
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">
                    Job Type <span class="text-error">*</span>
                </legend>
                <div class="flex gap-2">
                    {#each jobTypes as type}
                        <label
                            class="transition-smooth flex flex-1 cursor-pointer items-center justify-center gap-2 rounded-lg border-2 px-4 py-3 {formData.jobType ===
                            type
                                ? 'border-primary bg-primary/10 text-primary'
                                : 'border-white/10 hover:border-white/30'}"
                        >
                            <input
                                type="radio"
                                name="jobType"
                                value={type}
                                bind:group={formData.jobType}
                                class="hidden"
                            />
                            <span class="text-sm font-medium">{JOB_TYPE_LABELS[type]}</span>
                        </label>
                    {/each}
                </div>
            </fieldset>

            <div class="divider"></div>

            <div class="grid grid-cols-2 gap-4">
                <fieldset class="fieldset">
                    <legend class="fieldset-legend gap-1">
                        Daily Hours <span class="text-error">*</span>
                    </legend>
                    <label
                        class="transition-smooth input w-full"
                        class:input-error={error?.field === 'daily_work_hours'}
                    >
                        <iconify-icon icon="mdi:clock-outline"></iconify-icon>
                        <input
                            type="number"
                            class="grow"
                            placeholder="8"
                            bind:value={formData.dailyWorkHours}
                            min={1}
                            max={MAX_DAILY_WORK_HOURS}
                            required
                        />
                    </label>
                    <span class="label">Hours per day</span>
                </fieldset>

                <fieldset class="fieldset">
                    <legend class="fieldset-legend gap-1">
                        Days per Month <span class="text-error">*</span>
                    </legend>
                    <label
                        class="transition-smooth input w-full"
                        class:input-error={error?.field === 'workdays_per_month'}
                    >
                        <iconify-icon icon="mdi:calendar-month"></iconify-icon>
                        <input
                            type="number"
                            class="grow"
                            placeholder="20"
                            bind:value={formData.workdaysPerMonth}
                            min={1}
                            max={MAX_WORK_DAYS_PER_MONTH}
                            required
                        />
                    </label>
                    <span class="label">Days per month</span>
                </fieldset>
            </div>

            <div class="grid grid-cols-2 gap-3 rounded-lg bg-white/5 p-3">
                <div class="flex items-center gap-2 text-sm text-base-content/70">
                    <iconify-icon icon="mdi:clock-outline" class="text-primary"></iconify-icon>
                    <span>
                        Monthly: <strong class="text-primary">{monthlyHours}h</strong>
                    </span>
                </div>
                <div class="flex items-center gap-2 text-sm text-base-content/70">
                    <iconify-icon icon="mdi:cash" class="text-secondary"></iconify-icon>
                    <span>
                        Hourly: <strong class="text-secondary">{formatCurrency(hourlyRate)}</strong>
                    </span>
                </div>
            </div>

            <div class="my-6 h-8">
                {#if error}
                    <div
                        class="relative -mx-6 flex h-full items-center justify-center gap-2 bg-error text-center text-error-content"
                        transition:fade={{ duration: 200 }}
                    >
                        <iconify-icon icon="mdi:alert" class="inline-block"></iconify-icon>
                        <p class="text-sm">
                            {error.message} ({error.code})
                        </p>

                        <button
                            type="button"
                            class="btn absolute right-0 btn-square btn-sm btn-error"
                            onclick={clearError}
                            aria-label="Dismiss error"
                        >
                            <iconify-icon icon="mdi:close" class="text-lg"></iconify-icon>
                        </button>
                    </div>
                {/if}
            </div>

            <button
                type="submit"
                class="transition-smooth btn w-full btn-primary hover:scale-105"
                disabled={isLoading}
            >
                {#if isLoading}
                    <span class="loading loading-spinner"></span>
                    Adding...
                {:else}
                    <iconify-icon icon="mdi:briefcase-plus" class="text-lg"></iconify-icon>
                    Add Job
                {/if}
            </button>
        </form>
    </div>

    <form method="dialog" class="modal-backdrop">
        <button class="cursor-default">close</button>
    </form>
</dialog>
