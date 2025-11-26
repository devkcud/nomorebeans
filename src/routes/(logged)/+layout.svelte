<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import { authStore } from '$lib/stores/auth.svelte';
    import { onMount } from 'svelte';
    import Button from '$lib/components/Button.svelte';
    import { slide } from 'svelte/transition';

    let { children } = $props();

    onMount(() => {
        if (!authStore.isLoggedIn) {
            goto('/');
        }
    });

    let currentProfile = $derived(authStore.currentProfile);
    let currentPath = $derived(page.url.pathname);

    type NavItem = (Folder | Path) & { id: number };

    type Folder = {
        name: string;
        icon: string;
        items?: NavItem[];
    };

    type Path = {
        path: string;
        label: string;
        icon: string;
        newTab?: boolean;
    };

    const navItems: NavItem[] = [
        {
            id: 1,
            path: '/dashboard',
            label: 'Dashboard',
            icon: 'mdi:view-dashboard'
        },
        {
            id: 2,
            path: '/jobs',
            label: 'Jobs',
            icon: 'mdi:briefcase'
        }
    ];

    let expandedFolders = $state<Set<string>>(new Set([]));

    function toggleFolder(folderName: string) {
        if (expandedFolders.has(folderName)) {
            expandedFolders.delete(folderName);
        } else {
            expandedFolders.add(folderName);
        }
        expandedFolders = new Set(expandedFolders);
    }

    function isActive(path: string) {
        return currentPath.startsWith(path);
    }

    function navigateTo(path: string) {
        goto(path);
    }

    function handleLogout() {
        authStore.logout();
        goto('/');
    }
</script>

{#if authStore.isLoggedIn}
    <div class="flex h-[calc(100vh-40px)] overflow-hidden">
        <aside class="flex w-64 flex-col border-r border-white/10">
            <div class="flex items-center gap-3 p-4">
                <div class="avatar">
                    <div class="size-10 rounded-md ring-2 ring-primary/30">
                        <img src={currentProfile?.avatar} alt={currentProfile?.username} />
                    </div>
                </div>
                <div class="flex-1 overflow-hidden">
                    <span
                        class="truncate bg-linear-to-r from-primary to-secondary bg-clip-text text-sm font-semibold text-transparent"
                    >
                        {currentProfile?.displayName}
                    </span>
                    <p class="truncate text-xs opacity-60">@{currentProfile?.username}</p>
                </div>

                <Button
                    icon="mdi:logout"
                    size="xs"
                    layout="circle"
                    color="error"
                    soft
                    onclick={handleLogout}
                />
            </div>

            {#snippet renderFolder(folder: Folder)}
                {#if expandedFolders.has(folder.name)}
                    <div
                        class="absolute top-8 bottom-0 w-px translate-x-4 bg-white/10"
                        transition:slide={{ duration: 200 }}
                    ></div>
                {/if}

                <button
                    onclick={() => toggleFolder(folder.name)}
                    class="group relative flex w-full items-center gap-2 rounded-lg px-2 py-2 text-left text-xs font-semibold tracking-wider text-base-content/60 uppercase transition-all duration-200 hover:bg-white/5 hover:text-base-content/90 hover:shadow-sm"
                >
                    <iconify-icon
                        icon="mdi:chevron-right"
                        class="text-base transition-transform duration-200 group-hover:scale-110"
                        style="transform: rotate({expandedFolders.has(folder.name) ? 90 : 0}deg);"
                    ></iconify-icon>
                    <iconify-icon
                        icon={folder.icon}
                        class="text-base transition-colors duration-200 {expandedFolders.has(
                            folder.name
                        )
                            ? 'text-primary/70'
                            : 'text-base-content/50'} group-hover:text-primary"
                    ></iconify-icon>
                    <span class="transition-colors duration-200">{folder.name}</span>
                </button>

                {#if expandedFolders.has(folder.name)}
                    <ul transition:slide={{ duration: 200 }}>
                        {#each folder.items as item (item.id)}
                            {#if item instanceof Object && 'path' in item}
                                {@const path = item as Path}
                                <li class="pl-6">
                                    <Button
                                        icon={path.icon}
                                        color={isActive(path.path) ? 'colorful' : 'ghost'}
                                        width="full"
                                        size="sm"
                                        text="start"
                                        block
                                        onclick={() => navigateTo(path.path)}
                                    >
                                        {path.label}
                                    </Button>
                                </li>
                            {:else}
                                {@const folder = item as Folder}
                                <li class="relative pl-6">
                                    {@render renderFolder(folder)}
                                </li>
                            {/if}
                        {/each}
                    </ul>
                {/if}
            {/snippet}

            <nav class="no-scrollbar flex-1 overflow-y-auto">
                <div class="flex items-center justify-between bg-white/10 px-2 py-1">
                    <h2 class="text-xs tracking-wider text-base-content/60">Navigation</h2>
                </div>

                <ul class="p-4">
                    {#each navItems as item}
                        {#if item instanceof Object && 'path' in item}
                            {@const path = item as Path}
                            <li>
                                <Button
                                    icon={path.icon}
                                    color={isActive(path.path) ? 'colorful' : 'ghost'}
                                    width="full"
                                    size="sm"
                                    text="start"
                                    onclick={() => navigateTo(path.path)}
                                >
                                    {path.label}
                                </Button>
                            </li>
                        {:else}
                            <li class="relative">
                                {@render renderFolder(item)}
                            </li>
                        {/if}
                    {/each}
                </ul>
            </nav>
        </aside>

        <main class="no-scrollbar flex-1 overflow-auto border-t border-white/10">
            <div class="relative min-h-full overflow-hidden">
                <div class="absolute inset-x-0 top-0 h-128 bg-linear-to-b from-primary/10"></div>

                <div class="relative space-y-6 p-8">
                    {@render children()}
                </div>
            </div>
        </main>
    </div>
{:else}
    <div class="flex h-[calc(100vh-40px)] items-center justify-center">
        <div class="flex flex-col items-center gap-4">
            <span class="loading loading-lg loading-spinner"></span>
            <p class="text-lg">Redirecting to login...</p>
        </div>
    </div>
{/if}
