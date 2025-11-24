<script lang="ts">
    import { openUrl } from '@tauri-apps/plugin-opener';
    import { page } from '$app/state';
    import Title from '$lib/components/Title.svelte';
    import Button from '$lib/components/Button.svelte';
    import { GITHUB_ISSUES_URL } from '$lib/constants';

    let status = $derived(page.status);

    const errorConfig = $derived.by(() => {
        switch (status) {
            case 404:
                return {
                    icon: 'mdi:compass-off',
                    title: 'Page Not Found',
                    description: "The page you're looking for seems to have wandered off."
                };
            case 403:
                return {
                    icon: 'mdi:shield-lock-outline',
                    title: 'Access Denied',
                    description: "You don't have permission to view this content."
                };
            case 500:
                return {
                    icon: 'mdi:server-network-off',
                    title: 'Server Error',
                    description: 'Our servers are having a moment. Please try again soon.'
                };
            default:
                return {
                    icon: 'mdi:alert-circle-outline',
                    title: 'Oops! Something went wrong',
                    description: 'An unexpected error occurred.'
                };
        }
    });
</script>

<div class="relative flex min-h-[calc(100vh-40px)] flex-col overflow-hidden">
    <div class="absolute -inset-x-48 top-1/5 h-screen rounded-t-full bg-secondary/5 blur-3xl"></div>

    <main class="relative flex grow flex-col items-center justify-center gap-4 p-8">
        <iconify-icon icon={errorConfig.icon} class="text-7xl text-error"></iconify-icon>

        <div class="text-center">
            <Title header={1} size="4xl" colored bold>
                {errorConfig.title}
            </Title>
            <p class="mt-2 text-center text-base-content/60">
                {errorConfig.description}
            </p>
        </div>

        <div class="my-4 flex w-full justify-center gap-2">
            <Button
                icon="mdi:chevron-left"
                color="colorful"
                width="wide"
                onclick={() => history.back()}
            >
                Go Back
            </Button>
        </div>

        <p class="mt-8 text-sm text-base-content/40">
            If this problem persists,
            <button onclick={() => openUrl(GITHUB_ISSUES_URL)} class="link link-info">
                open an issue on GitHub
                <iconify-icon icon="mdi:open-in-new" class="inline-block align-middle text-sm"
                ></iconify-icon>
            </button>.
        </p>
    </main>
</div>
