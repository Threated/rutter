<script lang="ts">
    import type { User } from "src/types";

    export let isHovered: boolean = false;
    export let user: User;
    let selfHoverd: boolean = false;
</script>

{#if isHovered || selfHoverd}
    <div
        class="user"
        on:mouseenter={() => (selfHoverd = true)}
        on:mouseleave={() => (selfHoverd = false)}
    >
        <h3>{user.name}</h3>
        <div class="gray">@{user.name}</div>
        <div class="follows">
            <div>
                {user.follows}<span class="gray">Follows</span>
            </div>
            <div>
                {user.follower}<span class="gray">Follower</span>
            </div>
        </div>
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
        color: var(--color-white);
        z-index: 50;
        gap: .5rem;
    }
    h3 {
        margin: 0;
    }
    .gray {
        color: gray;
    }
    span.gray {
        margin-left: 3px;
    }
    .follows {
        display: flex;
        gap: 0.5rem;
    }
</style>
