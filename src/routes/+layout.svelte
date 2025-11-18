<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { goto } from '$app/navigation';
    import { browser } from '$app/environment';
    import 'iconify-icon';
    import '../app.css';
    import Button from '$lib/components/Button.svelte';

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
    class="fixed top-0 right-0 left-0 z-50 grid h-10 grid-cols-3 items-center gap-2 bg-base-200 px-4 select-none"
    data-tauri-drag-region
>
    <h1>{#await window.title() then title}{title}{/await}</h1>

    <div data-tauri-drag-region></div>

    <section class="justify-self-end" data-tauri-drag-region>
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

<nav class="navbar fixed top-8">
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

    <label class="btn swap btn-square swap-rotate btn-sm">
        <input type="checkbox" class="theme-controller hidden" value="light" />

        <iconify-icon icon="bxs:sun" class="swap-off"></iconify-icon>
        <iconify-icon icon="bxs:moon" class="swap-on"></iconify-icon>
    </label>
</nav>

<div class="h-screen overflow-auto pt-10">
    {@render children()}
</div>
