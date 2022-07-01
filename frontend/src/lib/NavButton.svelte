<script lang="ts">
    import { useLocation, useNavigate } from "svelte-navigator";
    const location = useLocation();
    const navigate = useNavigate();
    export let icon;
    export let to: string | (() => void);
    const onClick = () => {
        if (typeof to === 'function') {
            to();
        } else {
            navigate(to);
        }
    }
    $: width = $location.pathname === to ? 35 : 20;
</script>

<button on:click={onClick}>
    <svelte:component this={icon} width=2.5rem stroke-width={width}/>
    <slot />
</button>

<style>
    button {
        display: flex;
        background: none;
        border: 0;
        border-radius: 1.5rem;
        align-items: center;
        gap: 0.5rem;
        font-size: 1.5rem;
        padding: 0.5rem;
    }
    button:hover {
        background-color: rgba(231, 233, 234, 0.2);
    }
</style>
