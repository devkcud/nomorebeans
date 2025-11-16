<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { goto } from '$app/navigation';
    import 'iconify-icon';
    import '../app.css';

    let { children } = $props();

    const window = getCurrentWindow();

    function logout() {
        goto('/');
    }
</script>

<div
    class="fixed top-0 right-0 left-0 z-50 grid h-10 grid-cols-3 items-center gap-2 bg-base-200 px-4 select-none"
    data-tauri-drag-region
>
    <div data-tauri-drag-region>
        {#await window.title() then title}{title}{/await}
    </div>

    <div data-tauri-drag-region></div>

    <div class="justify-self-end" data-tauri-drag-region>
        <button
            class="btn btn-circle btn-ghost btn-xs btn-warning"
            aria-label="minimize"
            onclick={() => window.minimize()}
        >
            <iconify-icon icon="mdi:window-minimize"></iconify-icon>
        </button>
        <button
            class="btn btn-circle btn-ghost btn-xs"
            aria-label="maximize"
            onclick={() => window.toggleMaximize()}
        >
            <iconify-icon icon="mdi:window-maximize"></iconify-icon>
        </button>
        <button
            class="btn btn-circle btn-ghost btn-xs btn-error"
            aria-label="close"
            onclick={() => window.close()}
        >
            <iconify-icon icon="mdi:window-close"></iconify-icon>
        </button>
    </div>
</div>

<header class="navbar fixed top-10">
    <button
        class="group btn btn-square btn-sm"
        aria-label="change profile"
        onclick={() => logout()}
    >
        <iconify-icon icon="mdi:users" class="text-sm"></iconify-icon>
    </button>

    <div class="grow"></div>

    <label class="btn swap btn-square swap-rotate btn-sm">
        <input type="checkbox" class="theme-controller hidden" value="light" />
        <iconify-icon icon="bxs:sun" class="swap-off"></iconify-icon>
        <iconify-icon icon="bxs:moon" class="swap-on"></iconify-icon>
    </label>
</header>

<div class="h-screen overflow-auto pt-10">
    {@render children()}
</div>
