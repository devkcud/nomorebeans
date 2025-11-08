<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import 'iconify-icon';
    import '../app.css';

    let { children } = $props();

    const window = getCurrentWindow();
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

<div class="h-screen overflow-auto pt-10">
    {@render children()}
</div>
