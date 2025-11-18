<script lang="ts">
    import { getVersion } from '@tauri-apps/api/app';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { goto } from '$app/navigation';
    import { browser } from '$app/environment';
    import 'iconify-icon';
    import '../app.css';
    import Button from '$lib/components/Button.svelte';
    import Title from '$lib/components/Title.svelte';

    let { children } = $props();

    const window = getCurrentWindow();

    function logout() {
        goto('/');
    }

    function previousPage() {
        if (browser) history.back();
    }

    function nextPage() {
        if (browser) history.forward();
    }
</script>

<header
    class="fixed top-0 right-0 left-0 z-50 grid h-10 grid-cols-3 items-center gap-2 px-4 select-none"
    data-tauri-drag-region
>
    <Title size="sm" colored bold>
        {#await window.title() then title}
            {title}
        {/await}
    </Title>

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

<nav class="navbar fixed top-8 z-40 rounded-2xl">
    <section class="flex items-center gap-2">
        <Button icon="mdi:users" size="sm" layout="square" onclick={logout} />

        <div class="join">
            <Button
                icon="mdi:arrow-left"
                size="sm"
                layout="square"
                onclick={previousPage}
                isJoinItem
            />
            <Button
                icon="mdi:arrow-right"
                size="sm"
                layout="square"
                onclick={nextPage}
                isJoinItem
            />
        </div>
    </section>

    <div class="grow"></div>
</nav>

<div class="h-screen overflow-auto pt-10">
    {@render children()}
</div>

<p class="fixed bottom-2 right-4 text-xs opacity-30 select-none">
    {#await getVersion() then version}
        v{version}
    {/await}
</p>
