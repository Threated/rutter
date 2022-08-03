<script lang="ts">
    import { isAuthenticated } from "../store";
    import { navigate } from "svelte-navigator";
    import SendTweet from "./SendTweet.svelte";
    import Tweet from "./Tweet.svelte";
    import type { Tweet as TweetT } from "../types";
    import { auth_fetch } from "../main";
    $: if (!$isAuthenticated) {
        // Replace with cool shit https://svelte.dev/repl/033e824fad0a4e34907666e7196caec4?version=3.48.0
        navigate("/login");
        // alert("Login befor you can see your Feed");
    }
    let tweets: TweetT[] = [];
    async function timeline() {
        const res = await auth_fetch("http://localhost:8000/user/timeline", {
            method: "GET",
        });
        if (res.ok) {
            tweets = await res.json();
            // console.log(tweets);
        } else {
            throw new Error(
                (await res.json()).message
            )
        }
    }
</script>


<main>
    <div>
        <h1>New Tweets</h1>
        <SendTweet on:tweet={(event) => {tweets = [event.detail.tweet, ...tweets]}} />
    </div>
{#await timeline()}
    <h3>Loading timeline...</h3>
{:then} 
    {#each tweets as tweet}
        <Tweet {tweet}/>    
    {/each}
{/await}
</main>

<style>
    div {
        padding: 1rem;
        border-bottom: 2px solid white;
    }
    h1 {
        margin-top: 0;
    }
</style>
