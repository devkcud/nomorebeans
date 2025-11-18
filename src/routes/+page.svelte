<script lang="ts">
    import { getProfiles } from '$lib/api/profile-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { GetProfileResponse } from '$lib/api/types/profile';
    import { toggleLayoutMode, type ProfileSelectionLayoutMode } from '$lib/api/types/settings';
    import ProfileCard from '$lib/components/ProfileCard.svelte';
    import CreateProfileModal from '$lib/components/CreateProfileModal.svelte';
    import Button from '$lib/components/Button.svelte';

    let createProfileModal = $state<CreateProfileModal>();

    let layout = $state<ProfileSelectionLayoutMode>('grid');
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

<div class="bg-linear-r flex min-h-[calc(100vh-40px)] flex-col from-base-100 to-base-300 p-8">
    <main class="flex grow flex-col items-center justify-center p-4">
        <header class="mb-8 flex items-center gap-4">
            <h2 class="text-2xl">Who's budgeting today?</h2>

            <div>
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
        </header>

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
                    class="tooltip btn size-32 btn-soft btn-primary"
                    data-tip="Add Profile"
                    aria-label="Add Profile"
                    onclick={() => createProfileModal.open()}
                >
                    <iconify-icon icon="mdi:plus" class="text-5xl"></iconify-icon>
                </button>
            </section>
        {:else}
            <section class="mb-6 flex w-full max-w-xl flex-col divide-y divide-neutral">
                {#each profiles as profile (profile.id)}
                    <ProfileCard {profile} variant="list" />
                {/each}
            </section>

            <Button icon="mdi:plus" color="primary" onclick={createProfileModal.open}>
                Add Profile
            </Button>
        {/if}

        <article class="mt-12">
            <Button icon="mdi:cog">Settings</Button>
            <Button icon="material-symbols:manage-accounts">Manage Profiles</Button>
            <Button icon="mdi:dollar" color="primary">EzPay</Button>
        </article>
    </main>
</div>

<CreateProfileModal bind:this={createProfileModal} onSuccess={handleProfileCreated} />
