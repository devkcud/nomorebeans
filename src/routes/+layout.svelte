<script lang="ts">
    import { getVersion } from '@tauri-apps/api/app';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { goto } from '$app/navigation';
    import 'iconify-icon';
    import '../app.css';
    import Button from '$lib/components/Button.svelte';
    import Title from '$lib/components/Title.svelte';

    let { children } = $props();

    const window = getCurrentWindow();

    function logout() {
        goto('/');
    }
</script>

<header
    class="fixed inset-x-0 top-0 z-50 grid h-10 grid-cols-3 items-center gap-2 px-3 select-none"
    data-tauri-drag-region
>
    <div class="flex items-center gap-1" data-tauri-drag-region>
        <Title size="sm" colored bold>
            {#await window.title() then title}
                {title}
            {/await}
        </Title>

        <Button
            icon="mdi:users"
            size="xs"
            layout="circle"
            color="ghost"
            tooltip="Change users"
            tooltipOrientation="bottom"
            onclick={logout}
        />
    </div>

    <div data-tauri-drag-region></div>

    <section class="flex gap-1 justify-self-end" data-tauri-drag-region>
        <Button
            icon="mdi:window-minimize"
            size="xs"
            color="ghost"
            layout="circle"
            onclick={window.minimize}
        />
        <Button
            icon="mdi:window-maximize"
            size="xs"
            color="ghost"
            layout="circle"
            onclick={window.toggleMaximize}
        />
        <Button
            icon="mdi:window-close"
            size="xs"
            color="ghost"
            layout="circle"
            onclick={window.close}
        />
    </section>
</header>

<div class="h-screen overflow-auto pt-10">
    {@render children()}
</div>

<p class="fixed right-4 bottom-2 text-xs opacity-30 select-none">
    {#await getVersion() then version}
        v{version}
    {/await}
</p>
