<script lang="ts">
    import type { User } from "src/types";
    import { visitUser } from "../util";
    import SearchIcon from "../assets/ion-icons/search.svg";
    let results: User[] = []
    let query = "";
    $: fetch(`http://localhost:8000/user/?query=${query}`)
        .then(res => res.json())
        .then(json => results = json || [])
</script>

<div class="search">
    <div class="bar">
        <SearchIcon stroke-width=40 width=30px />
        <input type="search" bind:value={query} placeholder="Search Rutter">
    </div>
    <hr />
    <div class="results">
        {#each results as user}
            <div class="user" on:click={() => visitUser(user)}>
                <h3>{user.name}</h3>
                <div class="info">
                    <span>Follower {user.follower}</span>
                    <span>Following {user.follows}</span>
                </div>
            </div>
        {/each}
    
    </div>
</div>


<style>
    .bar {
        color: var(--color-white);
        gap: .3rem;
        font-size: x-large;
        display: flex;
    }
    hr {
        width: 100%;
        margin: 0;
        background: var(--color-white);
    }
    .bar input {
        border-radius: 1rem;
        padding: 0 .5rem;
        color: var(--color-white);
        background: none;
        border: 2px solid var(--color-white);
    }

    .user {
        border: 1px solid var(--color-white);
        border-left: 0;
        border-right: 0;    
        padding: .5rem 3rem;
    }
    .user h3 {
        background: none;
        margin: 0;
    }

    .user:hover {
        background-color: rgba(91, 112, 131, 0.1);
    }

    .info {
        display: flex;
        color: gray;
        gap: 1rem;
    }

    .search {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding-top: 1rem;
    }
    .results {
        display: flex;
        width: 100%;
        flex-direction: column;
    }
</style>
