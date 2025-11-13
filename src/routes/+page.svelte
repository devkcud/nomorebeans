<script lang="ts">
    import { createProfile, getProfiles } from '$lib/api/profile-service';
    import type { Profile } from '$lib/api/types/profile';
    import { fade } from 'svelte/transition';

    let layout: 'grid' | 'list' = $state('grid');
    let profiles = $state<Array<Profile>>([]);

    let username = $state('');
    let displayName = $state<string>();
    let profilePicture: File | undefined = $state(undefined);
    let imagePreviewUrl: string = $derived(
        profilePicture
            ? URL.createObjectURL(profilePicture)
            : `https://api.dicebear.com/9.x/thumbs/svg?seed=${username}`
    );

    let createProfileModal: HTMLDialogElement;
    let createProfileForm: HTMLFormElement;
    let createProfileFileInput: HTMLInputElement;

    let createProfileError = $state<string | undefined>(undefined);

    async function handleCreateProfile(e: Event) {
        e.preventDefault();

        try {
            await createProfile({
                username,
                displayName,
                profilePicture
            });

            profiles = await getProfiles();

            createProfileModal.close();
            await handleCancelCreateProfile();
        } catch (err: any) {
            createProfileError = err;
        }
    }

    async function handleCancelCreateProfile() {
        await new Promise((resolve) => setTimeout(resolve, 300)); // HACK: wait for modal close animation

        createProfileError = undefined;
        username = '';
        displayName = undefined;
        profilePicture = undefined;
        createProfileForm.reset();
    }

    function handleFileChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const file = target.files?.[0];
        if (file) profilePicture = file;
    }

    function handleRemovePickedFile() {
        profilePicture = undefined;
        createProfileFileInput.value = '';
    }

    $effect(() => {
        (async () => {
            profiles = await getProfiles();
        })();
    });

    $effect(() => {
        if (profilePicture) {
            const url = URL.createObjectURL(profilePicture);
            imagePreviewUrl = url;
            return () => URL.revokeObjectURL(url);
        }
    });
</script>

{#snippet maxLengthField(field: string, max: number)}
    <label for="" class="label" class:text-error={field.length > max}>{field.length}/{max}</label>
{/snippet}

<div class="bg-linear-r flex min-h-[calc(100vh-40px)] flex-col from-base-100 to-base-300 p-8">
    <main class="flex grow flex-col items-center justify-center p-4">
        <div class="mb-8 flex items-center gap-4">
            <h2 class="text-2xl">Who's budgeting today?</h2>

            <button
                class="btn swap btn-circle swap-rotate btn-ghost"
                aria-label="help"
                onclick={() => (layout = layout === 'grid' ? 'list' : 'grid')}
            >
                <input type="checkbox" class="hidden" checked={layout === 'list'} />

                <iconify-icon icon="mingcute:align-justify-fill" class="swap-on"></iconify-icon>
                <iconify-icon icon="mingcute:grid-fill" class="swap-off"></iconify-icon>
            </button>
        </div>

        {#if layout === 'grid'}
            <section class="flex max-w-5xl flex-wrap justify-center gap-6 *:shrink-0">
                {#each profiles as { id, username, display_name, avatar: avatar_url } (id)}
                    <button
                        class="flex cursor-pointer flex-col items-center space-y-2 text-center transition-transform hover:scale-105"
                    >
                        <img
                            src={avatar_url ??
                                `https://api.dicebear.com/9.x/thumbs/svg?seed=${username}`}
                            alt={username}
                            class="size-32 rounded-md bg-base-300 object-cover"
                        />

                        <div class="-space-y-1">
                            <h2 class="text-lg font-bold">
                                {display_name ?? username}
                            </h2>
                            <p class="text-sm">@{username}</p>
                        </div>
                    </button>
                {/each}

                <button
                    class="tooltip btn size-32 btn-soft btn-accent"
                    data-tip="Add Profile"
                    aria-label="Add Profile"
                    onclick={() => createProfileModal.showModal()}
                >
                    <iconify-icon icon="mdi:plus" class="text-5xl"></iconify-icon>
                </button>
            </section>
        {:else if layout === 'list'}
            <section class="flex w-full max-w-xl flex-col divide-y divide-base-300">
                {#await getProfiles() then profiles}
                    {#each profiles as { id, username, display_name, avatar } (id)}
                        <button
                            class="flex w-full cursor-pointer items-center gap-4 p-4 transition-colors hover:bg-base-200"
                        >
                            <img
                                src={avatar ??
                                    `https://api.dicebear.com/9.x/thumbs/svg?seed=${username}`}
                                alt={username}
                                class="size-16 rounded-md bg-base-300 object-cover"
                            />

                            <div class="text-left">
                                <h2 class="text-lg font-bold">{display_name ?? username}</h2>
                                <p class="text-sm opacity-70">@{username}</p>
                            </div>

                            <div class="grow"></div>

                            <div class="flex items-center gap-2">
                                <iconify-icon icon="mdi:chevron-right"></iconify-icon>
                            </div>
                        </button>
                    {/each}
                {/await}
            </section>

            <button
                class="btn m-4 mx-auto self-start btn-soft btn-accent"
                aria-label="Add Profile"
                onclick={() => createProfileModal.showModal()}
            >
                <iconify-icon icon="mdi:plus" class="text-2xl"></iconify-icon>
                <span class="ml-2">Add Profile</span>
            </button>
        {/if}

        <article class="mt-12">
            <button class="btn">
                <iconify-icon icon="mdi:cog" class="text-lg"></iconify-icon>
                Settings
            </button>
            <button class="btn">
                <iconify-icon icon="material-symbols:manage-accounts" class="text-lg"
                ></iconify-icon>
                Manage Profiles
            </button>
            <button class="btn btn-soft btn-accent">
                <iconify-icon icon="mdi:dollar" class="text-lg"></iconify-icon>
                EzPay
                <span class="badge badge-soft badge-success">new</span>
            </button>
        </article>
    </main>
</div>

<dialog class="modal" bind:this={createProfileModal} onclose={handleCancelCreateProfile}>
    <div class="modal-box w-full max-w-xl">
        <form method="dialog">
            <button
                class="btn absolute top-2 right-2 btn-circle btn-ghost btn-sm"
                aria-label="Close"
            >
                <iconify-icon icon="mdi:close"></iconify-icon>
            </button>
        </form>

        <form class="space-y-4" bind:this={createProfileForm} onsubmit={handleCreateProfile}>
            <h3 class="text-lg font-bold">Create New Profile</h3>

            <p class="pb-4 text-xs italic">
                <span class="text-error">*</span> required fields
            </p>

            {#if createProfileError}
                <div class="rounded-md bg-error p-2 text-center text-error-content">
                    <p>{createProfileError}</p>
                </div>
            {/if}

            <fieldset class="fieldset">
                <legend class="fieldset-legend gap-1">
                    Username <span class="text-error">*</span>
                </legend>
                <label class="input w-full">
                    <iconify-icon icon="mdi:user"></iconify-icon>
                    <input type="text" class="grow" placeholder="c00lus3r" bind:value={username} />
                </label>
                {@render maxLengthField(username, 16)}
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Display Name</legend>
                <label class="input w-full">
                    <iconify-icon icon="mdi:account"></iconify-icon>
                    <input
                        type="text"
                        class="grow"
                        placeholder={username ? username : 'Very Cool Name'}
                        bind:value={displayName}
                    />
                </label>
                {@render maxLengthField(displayName || username, 32)}
            </fieldset>

            <div class="divider"></div>

            <section class="flex items-center gap-6">
                <div class="group relative size-40">
                    <img
                        src={imagePreviewUrl}
                        alt="{username}'s avatar"
                        class="mask size-40 mask-squircle object-cover"
                    />
                    {#if profilePicture}
                        <button
                            type="button"
                            class="btn absolute -right-1 -bottom-1 btn-circle btn-error"
                            onclick={handleRemovePickedFile}
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
                            onchange={handleFileChange}
                            bind:this={createProfileFileInput}
                        />
                        <label for="" class="label">Max size 2MB</label>
                    </fieldset>
                </div>
            </section>

            <button type="submit" class="btn mt-8 w-full btn-primary">
                <iconify-icon icon="mdi:account-plus" class="text-lg"></iconify-icon>
                Create Profile
            </button>
        </form>
    </div>

    <form method="dialog" class="modal-backdrop">
        <button class="cursor-default">close</button>
    </form>
</dialog>
