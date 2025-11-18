<script lang="ts">
    import { goto } from '$app/navigation';
    import { getProfiles, deleteProfile } from '$lib/api/profile-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { GetProfileResponse } from '$lib/api/types/profile';
    import Button from '$lib/components/Button.svelte';
    import Title from '$lib/components/Title.svelte';
    import CreateProfileModal from '$lib/components/CreateProfileModal.svelte';
    import EditProfileModal from '$lib/components/EditProfileModal.svelte';
    import { getPlaceholderAvatarUrl } from '$lib/utils';

    let createProfileModal = $state<CreateProfileModal>();
    let editProfileModal = $state<EditProfileModal>();

    let profiles = $state<GetProfileResponse[]>([]);
    let isLoadingProfiles = $state<boolean>(true);
    let loadProfilesError = $state<ErrorResponse>();
    let deletingProfileId = $state<number | null>(null);
    let profileToDelete = $state<GetProfileResponse | null>(null);

    let deleteConfirmModal = $state<HTMLDialogElement>();

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

    function handleProfileCreated() {
        loadProfiles();
    }

    function handleProfileUpdated() {
        loadProfiles();
    }

    function handleEditProfile(profile: GetProfileResponse) {
        editProfileModal?.open(profile);
    }

    function confirmDeleteProfile(profile: GetProfileResponse) {
        profileToDelete = profile;
        deleteConfirmModal?.showModal();
    }

    async function handleDeleteProfile() {
        if (!profileToDelete) return;

        deletingProfileId = profileToDelete.id;
        try {
            await deleteProfile(profileToDelete.id);
            deleteConfirmModal?.close();
            profileToDelete = null;
            await loadProfiles();
        } catch (err) {
            loadProfilesError = err as ErrorResponse;
        } finally {
            deletingProfileId = null;
        }
    }

    function cancelDelete() {
        profileToDelete = null;
        deleteConfirmModal?.close();
    }

    function goBack() {
        goto('/');
    }

    $effect(() => {
        loadProfiles();
    });
</script>

<div class="relative flex min-h-[calc(100vh-40px)] flex-col overflow-hidden">
    <div class="absolute -inset-x-48 top-1/5 h-screen rounded-t-full bg-secondary/5 blur-3xl"></div>

    <main class="relative flex grow flex-col items-center p-8">
        <section class="mb-8 flex w-full max-w-4xl items-center justify-between">
            <div class="flex items-center gap-4">
                <Button
                    icon="mdi:arrow-left"
                    size="sm"
                    color="ghost"
                    layout="square"
                    onclick={goBack}
                />
                <Title header={2} colored bold>Manage Profiles</Title>
            </div>

            <div class="flex gap-2">
                <Button
                    icon="mdi:refresh"
                    size="sm"
                    color="ghost"
                    layout="square"
                    onclick={loadProfiles}
                />
                <Button
                    icon="mdi:account-plus"
                    size="sm"
                    color="primary"
                    onclick={() => createProfileModal!.open()}
                >
                    Add Profile
                </Button>
            </div>
        </section>

        {#if isLoadingProfiles}
            <div class="flex items-center justify-center py-12">
                <span class="loading loading-lg loading-spinner"></span>
            </div>
        {:else if loadProfilesError}
            <div class="alert alert-error max-w-4xl">
                <iconify-icon icon="mdi:alert-circle" class="text-xl"></iconify-icon>
                <div>
                    <h3 class="font-bold">Failed to load profiles</h3>
                    <p class="text-sm">{loadProfilesError.message}</p>
                </div>
                <Button icon="mdi:refresh" size="sm" onclick={loadProfiles}>Retry</Button>
            </div>
        {:else if profiles.length === 0}
            <div class="flex flex-col items-center justify-center gap-6 py-12">
                <iconify-icon icon="mdi:account-off" class="text-6xl text-accent"></iconify-icon>
                <div class="text-center">
                    <h3 class="text-xl font-bold">No profiles yet</h3>
                    <p class="text-sm text-neutral-content/50">Create your first profile to get started</p>
                </div>
                <Button
                    icon="mdi:account-plus"
                    color="primary"
                    onclick={() => createProfileModal!.open()}
                >
                    Create Profile
                </Button>
            </div>
        {:else}
            <section class="w-full max-w-4xl">
                <div class="glass-effect overflow-hidden rounded-2xl">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>Avatar</th>
                                <th>Username</th>
                                <th>Display Name</th>
                                <th class="text-right">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each profiles as profile (profile.id)}
                                <tr class="hover">
                                    <td>
                                        <div class="avatar">
                                            <div class="size-12 rounded-full ring-2 ring-primary/30">
                                                <img
                                                    src={profile.avatar ||
                                                        getPlaceholderAvatarUrl(profile.username)}
                                                    alt="{profile.username}'s avatar"
                                                />
                                            </div>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="flex items-center gap-2">
                                            <iconify-icon icon="mdi:user" class="text-primary"></iconify-icon>
                                            <span class="font-medium">{profile.username}</span>
                                        </div>
                                    </td>
                                    <td>
                                        {#if profile.displayName}
                                            <span>{profile.displayName}</span>
                                        {:else}
                                            <span class="italic text-neutral-content/60">None</span>
                                        {/if}
                                    </td>
                                    <td>
                                        <div class="flex justify-end gap-2">
                                            <Button
                                                icon="mdi:pencil"
                                                size="sm"
                                                color="ghost"
                                                layout="square"
                                                tooltip="Edit profile"
                                                onclick={() => handleEditProfile(profile)}
                                            />
                                            <Button
                                                icon="mdi:trash"
                                                size="sm"
                                                color="ghost"
                                                layout="square"
                                                tooltip="Delete profile"
                                                onclick={() => confirmDeleteProfile(profile)}
                                            />
                                        </div>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            </section>
        {/if}
    </main>
</div>

<CreateProfileModal bind:this={createProfileModal} onSuccess={handleProfileCreated} />
<EditProfileModal bind:this={editProfileModal} onSuccess={handleProfileUpdated} />

<dialog class="modal backdrop-blur-sm" bind:this={deleteConfirmModal}>
    <div class="glass-effect modal-box border border-error/20">
        <div
            class="pointer-events-none absolute inset-0 rounded-2xl bg-linear-to-br from-error/10 to-error/5"
        ></div>

        <form method="dialog">
            <button
                class="transition-smooth btn absolute top-2 right-2 z-10 btn-circle btn-ghost btn-sm hover:rotate-90 hover:bg-error/20"
                aria-label="Close modal"
            >
                <iconify-icon icon="mdi:close"></iconify-icon>
            </button>
        </form>

        <div class="relative z-10 space-y-4">
            <div class="flex items-center gap-3">
                <iconify-icon icon="mdi:alert-circle" class="text-4xl text-error"></iconify-icon>
                <h3 class="text-2xl font-bold text-error">Delete Profile</h3>
            </div>

            <p class="py-4">
                Are you sure you want to delete
                <strong>{profileToDelete?.displayName || profileToDelete?.username}</strong>? This
                action cannot be undone.
            </p>

            <div class="flex justify-end gap-2">
                <Button color="ghost" onclick={cancelDelete}>Cancel</Button>
                <Button
                    icon="mdi:trash"
                    color="error"
                    onclick={handleDeleteProfile}
                    disabled={deletingProfileId !== null}
                >
                    {#if deletingProfileId !== null}
                        <span class="loading loading-spinner"></span>
                        Deleting...
                    {:else}
                        Delete
                    {/if}
                </Button>
            </div>
        </div>
    </div>

    <form method="dialog" class="modal-backdrop">
        <button class="cursor-default">close</button>
    </form>
</dialog>
