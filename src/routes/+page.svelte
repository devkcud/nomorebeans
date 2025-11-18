<script lang="ts">
    import { getProfiles } from '$lib/api/profile-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { GetProfileResponse } from '$lib/api/types/profile';
    import { toggleLayoutMode, type ProfileSelectionLayoutMode } from '$lib/api/types/settings';
    import ProfileCard from '$lib/components/ProfileCard.svelte';
    import CreateProfileModal from '$lib/components/CreateProfileModal.svelte';
    import Button from '$lib/components/Button.svelte';
    import Title from '$lib/components/Title.svelte';

    let createProfileModal = $state<CreateProfileModal>();

    let layout = $state<ProfileSelectionLayoutMode>('list');
    let profiles = $state<GetProfileResponse[]>([]);
    let isLoadingProfiles = $state<boolean>(true);
    let loadProfilesError = $state<ErrorResponse>();

    async function loadProfiles() {
        isLoadingProfiles = true;
        loadProfilesError = undefined;

        try {
            profiles = await getProfiles();
        } catch (err) {
            loadProfilesError = err as ErrorResponse;
        } finally {
            isLoadingProfiles = false;
        }
    }

    function toggleLayout() {
        layout = toggleLayoutMode(layout);
    }

    function handleProfileCreated() {
        loadProfiles();
    }

    $effect(() => {
        loadProfiles();
    });
</script>

<div class="relative flex min-h-[calc(100vh-40px)] flex-col overflow-hidden">
    <div class="absolute -inset-x-48 top-1/5 h-screen rounded-t-full bg-secondary/5 blur-3xl"></div>

    <main class="relative flex grow flex-col items-center justify-center p-4">
        <section class="mb-12 flex items-center gap-6">
            <Title header={2} colored bold>Who's budgeting today?</Title>

            <div class="flex gap-1">
                <Button
                    icon="mdi:refresh"
                    size="sm"
                    color="ghost"
                    layout="square"
                    onclick={loadProfiles}
                />
                <Button
                    icon={layout === 'grid' ? 'mingcute:grid-fill' : 'mingcute:align-justify-fill'}
                    size="sm"
                    color="ghost"
                    layout="square"
                    onclick={toggleLayout}
                />
            </div>
        </section>

        {#if isLoadingProfiles}
            <div class="flex items-center justify-center py-12">
                <span class="loading loading-lg loading-spinner"></span>
            </div>
        {:else if loadProfilesError}
            <div class="alert alert-error">
                <iconify-icon icon="mdi:alert-circle" class="text-xl"></iconify-icon>
                <div>
                    <h3 class="font-bold">Failed to load profiles</h3>
                    <p class="text-sm">{loadProfilesError.message}</p>
                </div>
                <Button icon="mdi:refresh" size="sm" onclick={loadProfiles}>Retry</Button>
            </div>
        {:else if layout === 'grid'}
            <section class="flex max-w-5xl flex-wrap justify-center gap-6 *:shrink-0">
                {#each profiles as profile (profile.id)}
                    <ProfileCard {profile} variant="grid" />
                {/each}

                <button
                    class="group transition-smooth glass-effect relative flex size-44 cursor-pointer flex-col items-center justify-center rounded-2xl p-4 text-center hover:-translate-y-2 hover:bg-linear-to-br hover:from-primary/10 hover:to-secondary/10"
                    aria-label="Add Profile"
                    onclick={() => createProfileModal!.open()}
                >
                    <div
                        class="transition-smooth absolute inset-0 rounded-2xl bg-linear-to-br from-primary/5 to-secondary/5 opacity-0 group-hover:opacity-100"
                    ></div>
                    <iconify-icon
                        icon="mdi:plus"
                        class="transition-smooth relative translate-y-4 text-6xl text-primary group-hover:translate-y-0"
                    ></iconify-icon>
                    <span
                        class="transition-smooth relative mt-2 text-sm font-medium opacity-0 group-hover:opacity-100"
                    >
                        Add Profile
                    </span>
                </button>
            </section>
        {:else}
            <section class="mb-6 flex w-full max-w-xl flex-col divide-y divide-neutral">
                <div>
                    {#each profiles as profile (profile.id)}
                        <ProfileCard {profile} variant="list" />
                    {/each}
                </div>

                <button
                    class="group transition-smooth glass-effect relative flex h-12 w-full cursor-pointer items-center justify-center gap-2 rounded-2xl p-4 text-center hover:scale-105 hover:bg-linear-to-br hover:from-primary/10 hover:to-secondary/10"
                    onclick={() => createProfileModal!.open()}
                >
                    <div
                        class="transition-smooth absolute inset-0 rounded-2xl bg-linear-to-br from-primary/5 to-secondary/5 opacity-0 group-hover:opacity-100"
                    ></div>
                    <iconify-icon
                        icon="mdi:plus"
                        class="transition-smooth relative text-2xl text-primary"
                    ></iconify-icon>
                    <span class="transition-smooth relative text-sm font-medium">
                        Add Profile
                    </span>
                </button>
            </section>
        {/if}

        <article class="mt-16 flex gap-2">
            <Button icon="mdi:cog" color="ghost">Settings</Button>
            <Button icon="mdi:account-cog" color="ghost">Manage Users</Button>
            <Button icon="mdi:dollar" color="primary">EzPay</Button>
        </article>
    </main>
</div>

<CreateProfileModal bind:this={createProfileModal} onSuccess={handleProfileCreated} />
