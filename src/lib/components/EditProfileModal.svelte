<script lang="ts">
    import { updateProfile } from '$lib/api/profile-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { UpdateProfileRequest, GetProfileResponse } from '$lib/api/types/profile';
    import {
        MAX_USERNAME_LENGTH,
        MAX_DISPLAY_NAME_LENGTH,
        MODAL_CLOSE_ANIMATION_MS
    } from '$lib/constants';
    import { getPlaceholderAvatarUrl } from '$lib/utils';
    import { fade } from 'svelte/transition';

    interface Props {
        onSuccess?: () => void;
    }

    let { onSuccess }: Props = $props();

    let currentProfile = $state<GetProfileResponse | null>(null);
    let formData = $state<UpdateProfileRequest>({
        username: undefined,
        displayName: undefined,
        profilePicture: undefined
    });

    let isLoading = $state(false);
    let error = $state<ErrorResponse | undefined>(undefined);

    let modalElement: HTMLDialogElement;
    let formElement: HTMLFormElement;
    let fileInputElement: HTMLInputElement;

    let imagePreviewUrl = $state<string>('');

    $effect(() => {
        if (formData.profilePicture) {
            const url = URL.createObjectURL(formData.profilePicture);
            imagePreviewUrl = url;
            return () => URL.revokeObjectURL(url);
        } else if (currentProfile) {
            imagePreviewUrl =
                currentProfile.avatar ||
                getPlaceholderAvatarUrl(formData.username || currentProfile.username);
        }
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        if (!currentProfile) return;

        isLoading = true;

        try {
            await updateProfile(currentProfile.id, formData);
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
        currentProfile = null;
        formData = {
            username: undefined,
            displayName: undefined,
            profilePicture: undefined
        };
        formElement?.reset();
    }

    function handleFileChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const file = target.files?.[0];
        if (file) {
            formData.profilePicture = file;
        }
    }

    function handleRemoveFile() {
        formData.profilePicture = undefined;
        if (fileInputElement) {
            fileInputElement.value = '';
        }
    }

    function clearError() {
        error = undefined;
    }

    export function open(profile: GetProfileResponse) {
        currentProfile = profile;
        formData = {
            username: profile.username,
            displayName: profile.displayName,
            profilePicture: undefined
        };
        modalElement?.showModal();
    }
</script>

{#snippet maxLengthIndicator(value: string, max: number)}
    <span class="label" class:text-error={value.length > max}>
        {value.length}/{max}
    </span>
{/snippet}

<dialog class="modal backdrop-blur-sm" bind:this={modalElement} onclose={resetForm}>
    <div class="glass-effect modal-box w-full max-w-xl border border-primary/20">
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
                Edit Profile
            </h3>

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">Username</legend>
                <label
                    class="input input-ghost transition-smooth w-full"
                    class:input-error={error?.field === 'username'}
                >
                    <iconify-icon icon="mdi:user"></iconify-icon>
                    <input
                        type="text"
                        class="grow"
                        placeholder="c00lus3r"
                        bind:value={formData.username}
                        maxlength={MAX_USERNAME_LENGTH}
                    />
                </label>
                {@render maxLengthIndicator(formData.username || '', MAX_USERNAME_LENGTH)}
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Display Name</legend>
                <label
                    class="input input-ghost transition-smooth w-full"
                    class:input-error={error?.field === 'display_name'}
                >
                    <iconify-icon icon="mdi:account"></iconify-icon>
                    <input
                        type="text"
                        class="grow"
                        placeholder={formData.username || 'Very Cool Name'}
                        bind:value={formData.displayName}
                        maxlength={MAX_DISPLAY_NAME_LENGTH}
                    />
                </label>
                {@render maxLengthIndicator(
                    formData.displayName || formData.username || '',
                    MAX_DISPLAY_NAME_LENGTH
                )}
            </fieldset>

            <div class="divider"></div>

            <section class="flex items-center gap-6">
                <div class="group relative size-40">
                    <div
                        class="transition-smooth absolute inset-0 rounded-3xl bg-linear-to-br from-primary to-secondary opacity-20 blur-xl group-hover:opacity-40"
                    ></div>
                    <img
                        src={imagePreviewUrl}
                        alt="{formData.username || currentProfile?.username}'s avatar"
                        class="transition-smooth relative size-40 rounded-3xl object-cover ring-4 ring-primary/30 group-hover:scale-105 group-hover:ring-primary/60"
                    />
                    {#if formData.profilePicture}
                        <button
                            type="button"
                            class="transition-bounce btn absolute -right-2 -bottom-2 btn-circle btn-error hover:scale-110"
                            onclick={handleRemoveFile}
                            aria-label="Remove avatar"
                            transition:fade={{ duration: 200 }}
                        >
                            <iconify-icon icon="mdi:trash" class="text-xl"></iconify-icon>
                        </button>
                    {/if}
                </div>

                <div>
                    <fieldset class="fieldset grow">
                        <legend class="fieldset-legend">Upload new avatar</legend>
                        <input
                            type="file"
                            accept="image/*"
                            class="file-input file-input-ghost w-full"
                            class:file-input-error={error?.field === 'profile_picture_bytes'}
                            onchange={handleFileChange}
                            bind:this={fileInputElement}
                        />
                        <span class="label">Max size 2 MB</span>
                    </fieldset>
                </div>
            </section>

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
                    Updating...
                {:else}
                    <iconify-icon icon="mdi:content-save" class="text-lg"></iconify-icon>
                    Save Changes
                {/if}
            </button>
        </form>
    </div>

    <form method="dialog" class="modal-backdrop">
        <button class="cursor-default">close</button>
    </form>
</dialog>
