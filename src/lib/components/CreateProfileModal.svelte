<script lang="ts">
    import { createProfile } from '$lib/api/profile-service';
    import type { ErrorResponse } from '$lib/api/types/error';
    import type { CreateProfileRequest } from '$lib/api/types/profile';
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

    let formData = $state<CreateProfileRequest>({
        username: '',
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
        } else {
            imagePreviewUrl = getPlaceholderAvatarUrl(formData.username || 'default');
        }
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        isLoading = true;

        try {
            await createProfile(formData);
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
            username: '',
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

    export function open() {
        modalElement?.showModal();
    }
</script>

{#snippet maxLengthIndicator(value: string, max: number)}
    <span class="label" class:text-error={value.length > max}>
        {value.length}/{max}
    </span>
{/snippet}

<dialog class="modal" bind:this={modalElement} onclose={resetForm}>
    <div class="modal-box w-full max-w-xl">
        <form method="dialog">
            <button
                class="btn absolute top-2 right-2 btn-circle btn-ghost btn-sm"
                aria-label="Close modal"
            >
                <iconify-icon icon="mdi:close"></iconify-icon>
            </button>
        </form>

        <form class="space-y-4" bind:this={formElement} onsubmit={handleSubmit}>
            <h3 class="text-lg font-bold">Create New Profile</h3>

            <p class="pb-4 text-xs italic">
                <span class="text-error">*</span> required fields
            </p>

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">
                    Username <span class="text-error">*</span>
                </legend>
                <label class="input w-full" class:input-error={error?.field === 'username'}>
                    <iconify-icon icon="mdi:user"></iconify-icon>
                    <input
                        type="text"
                        class="grow"
                        placeholder="c00lus3r"
                        bind:value={formData.username}
                        maxlength={MAX_USERNAME_LENGTH}
                        required
                    />
                </label>
                {@render maxLengthIndicator(formData.username, MAX_USERNAME_LENGTH)}
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Display Name</legend>
                <label class="input w-full" class:input-error={error?.field === 'display_name'}>
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
                    formData.displayName || formData.username,
                    MAX_DISPLAY_NAME_LENGTH
                )}
            </fieldset>

            <div class="divider"></div>

            <section class="flex items-center gap-6">
                <div class="group relative size-40">
                    <img
                        src={imagePreviewUrl}
                        alt="{formData.username}'s avatar"
                        class="mask size-40 mask-squircle object-cover"
                    />
                    {#if formData.profilePicture}
                        <button
                            type="button"
                            class="btn absolute -right-1 -bottom-1 btn-circle btn-error"
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
                        <legend class="fieldset-legend">Upload an avatar</legend>
                        <input
                            type="file"
                            accept="image/*"
                            class="file-input w-full"
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

            <button type="submit" class="btn w-full btn-primary" disabled={isLoading}>
                {#if isLoading}
                    <span class="loading loading-spinner"></span>
                    Creating...
                {:else}
                    <iconify-icon icon="mdi:account-plus" class="text-lg"></iconify-icon>
                    Create Profile
                {/if}
            </button>
        </form>
    </div>

    <form method="dialog" class="modal-backdrop">
        <button class="cursor-default">close</button>
    </form>
</dialog>
