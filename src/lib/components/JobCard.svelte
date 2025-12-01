<script lang="ts">
    import type { Job } from '$lib/api/types/job';
    import { JOB_TYPE_LABELS, JOB_TYPE_ICONS } from '$lib/api/types/job';
    import Button from './Button.svelte';

    interface Props {
        job: Job;
        onEdit?: () => void;
        onDelete?: () => void;
    }

    let { job, onEdit, onDelete }: Props = $props();

    let monthlyHours = $derived(job.dailyWorkHours * job.workdaysPerMonth);

    function formatCurrency(value: number): string {
        return new Intl.NumberFormat('pt-BR', {
            style: 'currency',
            currency: 'BRL'
        }).format(value);
    }

    let hourlyRate = $derived(monthlyHours > 0 ? job.salaryGross / monthlyHours : 0);
</script>

<article
    class="glass-effect group relative overflow-hidden rounded-2xl border border-white/10 p-5 transition-all duration-300 hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5"
>
    <div
        class="pointer-events-none absolute inset-0 bg-linear-to-br from-primary/5 via-transparent to-secondary/5 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
    ></div>

    <div class="relative">
        <header class="mb-4 flex items-start justify-between">
            <div class="flex items-center gap-3">
                <div
                    class="flex size-12 items-center justify-center rounded-xl bg-linear-to-br from-primary/20 to-secondary/20"
                >
                    <iconify-icon
                        icon={JOB_TYPE_ICONS[job.jobType]}
                        class="text-2xl text-primary"
                    ></iconify-icon>
                </div>
                <div>
                    <h3 class="font-semibold text-base-content">{job.positionTitle}</h3>
                    <p class="text-sm text-base-content/60">{job.companyName}</p>
                </div>
            </div>

            <span
                class="rounded-full bg-primary/10 px-3 py-1 text-xs font-medium text-primary"
            >
                {JOB_TYPE_LABELS[job.jobType]}
            </span>
        </header>

        <div class="mb-3 flex items-center justify-between rounded-lg bg-linear-to-r from-primary/10 to-secondary/10 px-4 py-3">
            <div>
                <p class="text-xs text-base-content/50">Gross Salary</p>
                <p class="text-xl font-bold text-primary">{formatCurrency(job.salaryGross)}</p>
            </div>
            <div class="text-right">
                <p class="text-xs text-base-content/50">Hourly Rate</p>
                <p class="text-lg font-semibold text-secondary">{formatCurrency(hourlyRate)}/h</p>
            </div>
        </div>

        <div class="mb-4 grid grid-cols-3 gap-3">
            <div class="rounded-lg bg-white/5 p-3 text-center">
                <iconify-icon icon="mdi:clock-outline" class="text-lg text-primary/70"></iconify-icon>
                <p class="mt-1 text-lg font-semibold">{job.dailyWorkHours}h</p>
                <p class="text-xs text-base-content/50">Daily</p>
            </div>
            <div class="rounded-lg bg-white/5 p-3 text-center">
                <iconify-icon icon="mdi:calendar-week" class="text-lg text-primary/70"></iconify-icon>
                <p class="mt-1 text-lg font-semibold">{job.workdaysPerMonth}</p>
                <p class="text-xs text-base-content/50">Days/Month</p>
            </div>
            <div class="rounded-lg bg-white/5 p-3 text-center">
                <iconify-icon icon="mdi:sigma" class="text-lg text-primary/70"></iconify-icon>
                <p class="mt-1 text-lg font-semibold">{monthlyHours}h</p>
                <p class="text-xs text-base-content/50">Monthly</p>
            </div>
        </div>

        <footer class="flex justify-end gap-2">
            <Button
                icon="mdi:pencil"
                size="sm"
                color="ghost"
                onclick={onEdit}
            >
                Edit
            </Button>
            <Button
                icon="mdi:trash"
                size="sm"
                color="error"
                soft
                onclick={onDelete}
            >
                Delete
            </Button>
        </footer>
    </div>
</article>
