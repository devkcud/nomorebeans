<script lang="ts">
    import type { Snippet } from 'svelte';

    interface Props {
        children: Snippet<[]>;
        header?: number;
        size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl' | '5xl';
        colored?: boolean;
        bold?: boolean;
    }

    let { children, header = 2, size = '2xl', colored, bold }: Props = $props();

    let tag = $derived(`h${header}`);

    const sizeClasses = {
        xs: 'text-xs',
        sm: 'text-sm',
        md: 'text-base',
        lg: 'text-lg',
        xl: 'text-xl',
        '2xl': 'text-2xl',
        '3xl': 'text-3xl',
        '4xl': 'text-4xl',
        '5xl': 'text-5xl'
    };
</script>

<svelte:element
    this={tag}
    class="
    {colored
        ? `
        bg-linear-to-r
        from-primary
        to-secondary
        bg-clip-text
        text-transparent
    `
        : ''}
    {bold ? 'font-semibold' : ''}
    {size ? sizeClasses[size] : ''}
    "
>
    {@render children()}
</svelte:element>
