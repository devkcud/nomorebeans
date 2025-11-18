<script lang="ts">
    import type { GetProfileResponse } from '$lib/api/types/profile';
    import type { ProfileSelectionLayoutMode } from '$lib/api/types/settings';
    import { getAvatarUrl } from '$lib/utils';
    import Title from './Title.svelte';

    interface Props {
        profile: GetProfileResponse;
        variant?: ProfileSelectionLayoutMode;
        onclick?: () => void;
    }

    let { profile, variant = 'grid', onclick }: Props = $props();
</script>

{#if variant === 'grid'}
    <button
        class="group transition-smooth glass-effect relative flex w-44 cursor-pointer flex-col items-center gap-2 rounded-2xl p-4 text-center hover:-translate-y-2"
        {onclick}
    >
        <div class="relative">
            <img
                src={getAvatarUrl(profile.avatar, profile.username)}
                alt={profile.username}
                class="transition-smooth size-32 rounded-2xl bg-base-300 object-cover ring-2 ring-primary/20 group-hover:scale-105 group-hover:ring-primary/60 group-focus:scale-105"
            />
            <div
                class="transition-smooth absolute -right-2 -bottom-2 flex size-8 items-center justify-center rounded-full bg-linear-to-br from-success to-accent opacity-0 group-hover:opacity-100"
            >
                <iconify-icon icon="mdi:arrow-right" class="text-sm text-white"></iconify-icon>
            </div>
        </div>

        <article class="relative">
            <Title header={3} size="md" colored bold>
                {profile.displayName ?? profile.username}
            </Title>
            <p class="text-sm">@{profile.username}</p>
        </article>
    </button>
{:else if variant === 'list'}
    <button
        class="group transition-smooth glass-effect relative my-2 flex w-full cursor-pointer items-center gap-4 rounded-xl p-5 hover:scale-[1.02] hover:bg-linear-to-r hover:from-primary/10 hover:via-transparent hover:to-secondary/10"
        {onclick}
    >
        <div class="relative">
            <img
                src={getAvatarUrl(profile.avatar, profile.username)}
                alt={profile.username}
                class="transition-smooth size-16 rounded-xl bg-base-300 object-cover ring-2 ring-primary/20 group-hover:scale-105 group-hover:ring-primary/60"
            />
        </div>

        <div class="flex-1 text-left">
            <h2
                class="bg-linear-to-r from-primary to-secondary bg-clip-text text-lg font-bold text-transparent"
            >
                {profile.displayName ?? profile.username}
            </h2>
            <p class="text-sm opacity-70">@{profile.username}</p>
        </div>

        <div class="transition-smooth flex items-center gap-2 group-hover:translate-x-3">
            <iconify-icon icon="mdi:chevron-right" class="text-primary"></iconify-icon>
        </div>
    </button>
{/if}
