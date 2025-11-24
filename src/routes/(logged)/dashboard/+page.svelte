<script lang="ts">
    import Title from '$lib/components/Title.svelte';
    import { authStore } from '$lib/stores/auth.svelte';

    let currentProfile = $derived(authStore.currentProfile);
</script>

<main class="relative space-y-6 p-8">
    <header class="flex items-center justify-between">
        <div>
            <Title header={1} size="4xl" colored bold>
                Welcome back, {currentProfile?.displayName || currentProfile?.username}!
            </Title>
            <p class="mt-2 text-sm text-base-content/60">Here's your financial overview</p>
        </div>
    </header>
</main>
