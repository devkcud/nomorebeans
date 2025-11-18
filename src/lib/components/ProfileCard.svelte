<script lang="ts">
    import type { GetProfileResponse } from '$lib/api/types/profile';
    import type { ProfileSelectionLayoutMode } from '$lib/api/types/settings';
    import { getAvatarUrl } from '$lib/utils';

    interface Props {
        profile: GetProfileResponse;
        variant?: ProfileSelectionLayoutMode;
        onclick?: () => void;
    }

    let { profile, variant = 'grid', onclick }: Props = $props();
</script>

{#if variant === 'grid'}
    <button
        class="group flex cursor-pointer flex-col items-center space-y-2 text-center transition-transform hover:scale-110 focus:scale-110 focus:outline-0"
        {onclick}
    >
        <img
            src={getAvatarUrl(profile.avatar, profile.username)}
            alt={profile.username}
            class="size-32 rounded-md bg-base-300 object-cover transition group-hover:ring-4 group-hover:ring-primary group-focus:ring-4 group-focus:ring-primary"
        />

        <div class="-space-y-1">
            <h2 class="text-lg font-bold text-primary">
                {profile.displayName ?? profile.username}
            </h2>
            <p class="text-sm">@{profile.username}</p>
        </div>
    </button>
{:else if variant === 'list'}
    <button
        class="flex w-full cursor-pointer items-center gap-4 p-4 transition-colors hover:bg-base-200"
        {onclick}
    >
        <img
            src={getAvatarUrl(profile.avatar, profile.username)}
            alt={profile.username}
            class="size-16 rounded-md bg-base-300 object-cover"
        />

        <div class="text-left">
            <h2 class="text-lg font-bold text-primary">
                {profile.displayName ?? profile.username}
            </h2>
            <p class="text-sm">@{profile.username}</p>
        </div>

        <div class="grow"></div>

        <div class="flex items-center gap-2">
            <iconify-icon icon="mdi:chevron-right"></iconify-icon>
        </div>
    </button>
{/if}
