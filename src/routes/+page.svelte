<script lang="ts">
    import { getProfiles } from '$lib/api/profile-service';

    let layout: 'grid' | 'list' = $state('grid');
</script>

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
                {#await getProfiles() then profiles}
                    {#each profiles as { id, username, display_name, avatar_url } (id)}
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
                {/await}

                <button
                    class="tooltip btn size-32 btn-soft btn-accent"
                    data-tip="Add Profile"
                    aria-label="Add Profile"
                >
                    <iconify-icon icon="mdi:plus" class="text-5xl"></iconify-icon>
                </button>
            </section>
        {:else if layout === 'list'}
            <section class="flex w-full max-w-xl flex-col divide-y divide-base-300">
                {#await getProfiles() then profiles}
                    {#each profiles as { id, username, display_name, avatar_url } (id)}
                        <button
                            class="flex w-full cursor-pointer items-center gap-4 p-4 transition-colors hover:bg-base-200"
                        >
                            <img
                                src={avatar_url ??
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

            <button class="btn m-4 mx-auto self-start btn-soft btn-accent" aria-label="Add Profile">
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
