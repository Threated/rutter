<script lang="ts">
    import type { Tweet } from "src/types";
    export let tweets: Tweet[];
</script>

<div>
    {#each tweets as tweet}
        <div class="tweet">
            <h3>{tweet.author.name}</h3>
            <p>{tweet.content}</p>
        </div>
    {/each}
</div>
