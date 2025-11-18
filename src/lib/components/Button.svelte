<script lang="ts">
    import type { Snippet } from 'svelte';

    interface Props {
        children?: Snippet<[]>;

        buttonType?: 'button' | 'submit' | 'reset';
        width?: 'full' | 'fit' | 'wide';

        size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
        layout?: 'circle' | 'square';
        color?:
            | 'primary'
            | 'secondary'
            | 'success'
            | 'warning'
            | 'error'
            | 'info'
            | 'neutral'
            | 'ghost';
        icon?: string;
        tooltip?: string;
        tooltipOrientation?: 'top' | 'bottom' | 'left' | 'right';

        isJoinItem?: boolean;
        isSoft?: boolean;

        onclick?: () => void;
    }

    let {
        children,
        buttonType = 'button',
        width,
        size = 'md',
        layout,
        color,
        icon,
        tooltip,
        tooltipOrientation = 'top',
        isJoinItem,
        isSoft,
        onclick
    }: Props = $props();

    const widthClass = {
        full: 'w-full',
        auto: 'w-auto',
        fit: 'w-fit',
        wide: 'btn-wide'
    };

    const sizeClass = {
        xs: 'btn-xs',
        sm: 'btn-sm',
        md: 'btn-md',
        lg: 'btn-lg',
        xl: 'btn-xl'
    };

    const layoutClass = {
        circle: 'btn-circle',
        square: 'btn-square'
    };

    const colorClass = {
        primary: 'btn-primary',
        secondary: 'btn-secondary',
        success: 'btn-success',
        warning: 'btn-warning',
        error: 'btn-error',
        info: 'btn-info',
        neutral: 'btn-neutral',
        ghost: 'btn-ghost'
    };

    const tooltipOrientationClass = {
        top: 'tooltip-top',
        bottom: 'tooltip-bottom',
        left: 'tooltip-left',
        right: 'tooltip-right'
    };
</script>

<button
    type={buttonType}
    class="
        btn
        {width ? widthClass[width] : ''}
        {sizeClass[size]}
        {layout ? layoutClass[layout] : ''}
        {color ? colorClass[color] : ''}
        items-center
        gap-2
        {isJoinItem ? 'join-item' : ''}
        {isSoft ? 'btn-soft' : ''}
        {tooltip ? 'tooltip' : ''}
        {tooltip && tooltipOrientation ? tooltipOrientationClass[tooltipOrientation] : ''}
    "
    {onclick}
    data-tip={tooltip}
>
    {#if icon}
        <iconify-icon {icon}></iconify-icon>
    {/if}

    {#if children}
        {@render children()}
    {/if}
</button>
