<script lang="ts">
    import { getJobs, deleteJob } from '$lib/api/job-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { Job } from '$lib/api/types/job';
    import Button from '$lib/components/Button.svelte';
    import CreateJobModal from '$lib/components/CreateJobModal.svelte';
    import EditJobModal from '$lib/components/EditJobModal.svelte';
    import JobCard from '$lib/components/JobCard.svelte';
    import Title from '$lib/components/Title.svelte';
    import { authStore } from '$lib/stores/auth.svelte';
    import { fade, fly } from 'svelte/transition';
    import { flip } from 'svelte/animate';

    let currentProfile = $derived(authStore.currentProfile);

    let createJobModal = $state<CreateJobModal>();
    let editJobModal = $state<EditJobModal>();

    let jobs = $state<Job[]>([]);
    let isLoading = $state(true);
    let error = $state<ErrorResponse>();

    let deleteConfirmJob = $state<Job | null>(null);
    let isDeleting = $state(false);

    async function loadJobs() {
        if (!currentProfile) return;

        isLoading = true;
        error = undefined;

        try {
            jobs = await getJobs(currentProfile.id);
        } catch (err) {
            error = err as ErrorResponse;
        } finally {
            isLoading = false;
        }
    }

    async function handleDelete() {
        if (!deleteConfirmJob || !currentProfile) return;

        isDeleting = true;
        try {
            await deleteJob(currentProfile.id, deleteConfirmJob.id);
            deleteConfirmJob = null;
            await loadJobs();
        } catch (err) {
            error = err as ErrorResponse;
        } finally {
            isDeleting = false;
        }
    }

    function handleJobCreated() {
        loadJobs();
    }

    function handleJobUpdated() {
        loadJobs();
    }

    function openEditModal(job: Job) {
        editJobModal?.open(job);
    }

    function openDeleteConfirm(job: Job) {
        deleteConfirmJob = job;
    }

    $effect(() => {
        loadJobs();
    });

    let totalMonthlyHours = $derived(
        jobs.reduce((acc, job) => acc + job.dailyWorkHours * job.workdaysPerMonth, 0)
    );

    let totalDailyHours = $derived(jobs.reduce((acc, job) => acc + job.dailyWorkHours, 0));

    let totalGrossSalary = $derived(jobs.reduce((acc, job) => acc + job.salaryGross, 0));

    function formatCurrency(value: number): string {
        return new Intl.NumberFormat('pt-BR', {
            style: 'currency',
            currency: 'BRL'
        }).format(value);
    }
</script>

<header class="flex items-center justify-between">
    <div>
        <Title header={1} size="4xl" colored bold>
            Manage your jobs, {currentProfile?.displayName}!
        </Title>
        <p class="mt-2 text-sm text-base-content/60">
            Keep track of all your income sources in one place.
        </p>
    </div>

    <Button
        icon="mdi:briefcase-plus"
        color="primary"
        onclick={() => createJobModal?.open()}
    >
        Add Job
    </Button>
</header>

{#if jobs.length > 0}
    <section
        class="flex flex-wrap gap-4 justify-between items-center"
        transition:fade={{ duration: 200 }}
    >
        <div class="flex items-center gap-4">
            <div class="flex size-10 items-center justify-center rounded-xl bg-primary/20">
                <iconify-icon icon="mdi:briefcase-outline" class="text-xl text-primary"></iconify-icon>
            </div>
            <div>
                <p class="text-xs text-base-content/60">Total Jobs</p>
                <p class="text-xl font-bold text-primary">{jobs.length}</p>
            </div>
        </div>

        <div class="flex items-center gap-4">
            <div class="flex size-10 items-center justify-center rounded-xl bg-success/20">
                <iconify-icon icon="mdi:cash" class="text-xl text-success"></iconify-icon>
            </div>
            <div>
                <p class="text-xs text-base-content/60">Total Salary (Gross)</p>
                <p class="text-xl font-bold text-success">{formatCurrency(totalGrossSalary)}</p>
            </div>
        </div>

        <div class="flex items-center gap-4">
            <div class="flex size-10 items-center justify-center rounded-xl bg-success/20">
                <iconify-icon icon="mdi:cash" class="text-xl text-success"></iconify-icon>
            </div>
            <div>
                <p class="text-xs text-base-content/60">Total Salary (Net)</p>
                <!-- TODO: Calculate total net salary -->
                <p class="text-xl font-bold text-success">{formatCurrency(totalGrossSalary)}</p>
            </div>
        </div>

        <div class="flex items-center gap-4">
            <div class="flex size-10 items-center justify-center rounded-xl bg-secondary/20">
                <iconify-icon icon="mdi:clock-outline" class="text-xl text-secondary"></iconify-icon>
            </div>
            <div>
                <p class="text-xs text-base-content/60">Daily Hours</p>
                <p class="text-xl font-bold text-secondary">{totalDailyHours}h</p>
            </div>
        </div>

        <div class="flex items-center gap-4">
            <div class="flex size-10 items-center justify-center rounded-xl bg-accent/20">
                <iconify-icon icon="mdi:calendar-month" class="text-xl text-accent"></iconify-icon>
            </div>
            <div>
                <p class="text-xs text-base-content/60">Monthly Hours</p>
                <p class="text-xl font-bold text-accent">{totalMonthlyHours}h</p>
            </div>
        </div>
    </section>
{/if}

<section>
    {#if isLoading}
        <div class="flex items-center justify-center py-16">
            <span class="loading loading-lg loading-spinner text-primary"></span>
        </div>
    {:else if error}
        <div class="alert alert-error">
            <iconify-icon icon="mdi:alert-circle" class="text-xl"></iconify-icon>
            <div>
                <h3 class="font-bold">Failed to load jobs</h3>
                <p class="text-sm">{error.message}</p>
            </div>
            <Button icon="mdi:refresh" size="sm" onclick={loadJobs}>Retry</Button>
        </div>
    {:else if jobs.length === 0}
        <div
            class="glass-effect flex flex-col items-center justify-center rounded-2xl border border-dashed border-white/20 py-16"
            transition:fade={{ duration: 200 }}
        >
            <div class="mb-6 flex size-24 items-center justify-center rounded-full bg-primary/10">
                <iconify-icon icon="mdi:briefcase-outline" class="text-5xl text-primary/50"></iconify-icon>
            </div>
            <h3 class="mb-2 text-xl font-semibold">No jobs yet</h3>
            <p class="mb-6 text-base-content/60">Start by adding your first income source</p>
            <Button
                icon="mdi:briefcase-plus"
                color="primary"
                onclick={() => createJobModal?.open()}
            >
                Add Your First Job
            </Button>
        </div>
    {:else}
        <div class="grid gap-4 md:grid-cols-2">
            {#each jobs as job (job.id)}
                <div
                    animate:flip={{ duration: 300 }}
                    in:fly={{ y: 20, duration: 300 }}
                    out:fade={{ duration: 200 }}
                >
                    <JobCard
                        {job}
                        onEdit={() => openEditModal(job)}
                        onDelete={() => openDeleteConfirm(job)}
                    />
                </div>
            {/each}
        </div>
    {/if}
</section>

{#if currentProfile}
    <CreateJobModal
        bind:this={createJobModal}
        profileId={currentProfile.id}
        onSuccess={handleJobCreated}
    />
{/if}

{#if currentProfile}
    <EditJobModal bind:this={editJobModal} profileId={currentProfile.id} onSuccess={handleJobUpdated} />
{/if}

{#if deleteConfirmJob}
    <dialog class="modal modal-open backdrop-blur-sm">
        <div class="glass-effect modal-box border border-error/20">
            <h3 class="text-lg font-bold text-error">Delete Job</h3>
            <p class="py-4">
                Are you sure you want to delete <strong>{deleteConfirmJob.positionTitle}</strong> at
                <strong>{deleteConfirmJob.companyName}</strong>? This action cannot be undone.
            </p>
            <div class="modal-action">
                <Button color="ghost" onclick={() => (deleteConfirmJob = null)}>Cancel</Button>
                <Button color="error" icon="mdi:trash" onclick={handleDelete} disabled={isDeleting}>
                    {#if isDeleting}
                        <span class="loading loading-spinner loading-sm"></span>
                        Deleting...
                    {:else}
                        Delete
                    {/if}
                </Button>
            </div>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button class="cursor-default" onclick={() => (deleteConfirmJob = null)}>close</button>
        </form>
    </dialog>
{/if}
