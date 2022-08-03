<script lang="ts">
    import type { User } from "src/types";

    // let x: number, y: number;
    export let isHovered: boolean = false;
    export let user: User;
    let selfHoverd: boolean = false;
</script>

{#if isHovered || selfHoverd}
<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div class="user" on:mouseenter={() => selfHoverd = true} on:mouseleave={() => selfHoverd = false}>
    <h3>{user.name}</h3>
</div>
{/if}

<style>
    .user {
        position: absolute;
        background-color: black;
        box-shadow: rgba(255, 255, 255, 0.24) 0px 3px 8px;
        top: 100%;
        border-radius: 6px;
        padding: 1rem;
    }
    h3 {
        margin: 0;
    }
</style>
