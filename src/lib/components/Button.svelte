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
            | 'accent'
            | 'success'
            | 'warning'
            | 'error'
            | 'info'
            | 'neutral'
            | 'ghost'
            | 'glass'
            | 'colorful';
        icon?: string;
        tooltip?: string;
        tooltipOrientation?: 'top' | 'bottom' | 'left' | 'right';
        text?: 'center' | 'start' | 'end';

        join?: boolean;
        soft?: boolean;
        disabled?: boolean;
        block?: boolean;
        nobg?: boolean;

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
        text = 'center',
        join: isJoinItem,
        soft: isSoft,
        disabled = false,
        block = false,
        nobg = false,
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
        accent: 'btn-accent',
        success: 'btn-success',
        warning: 'btn-warning',
        error: 'btn-error',
        info: 'btn-info',
        neutral: 'btn-neutral',
        ghost: 'btn-ghost',
        glass: 'glass-effect text-base-content',
        colorful: 'bg-linear-to-r from-primary/20 to-secondary/20 text-primary'
    };

    const tooltipOrientationClass = {
        top: 'tooltip-top',
        bottom: 'tooltip-bottom',
        left: 'tooltip-left',
        right: 'tooltip-right'
    };

    const textClass = {
        center: 'justify-center',
        start: 'justify-start',
        end: 'justify-end'
    };
</script>

<button
    type={buttonType}
    class="
        btn
        font-normal
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
        transition-smooth
        hover:scale-105
        active:scale-95
        {text ? textClass[text] : ''}
        {block ? 'flex' : 'inline-flex'}
        {nobg ? 'border-0 bg-transparent hover:bg-transparent active:bg-transparent' : ''}
    "
    {onclick}
    data-tip={tooltip}
    {disabled}
>
    {#if icon}
        <iconify-icon {icon} class="transition-smooth"></iconify-icon>
    {/if}

    {#if children}
        {@render children()}
    {/if}
</button>
