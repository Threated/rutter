<script lang="ts">
    import { isAuthenticated } from "../store";
    import { useNavigate } from "svelte-navigator";
    import SendTweet from "./SendTweet.svelte";
    import TweetStream from "./TweetStream.svelte";
    import type { Tweet } from "src/types";
    const navigate = useNavigate();
    $: if (!$isAuthenticated) {
        // Replace with cool shit https://svelte.dev/repl/033e824fad0a4e34907666e7196caec4?version=3.48.0
        navigate("/login");
        // alert("Login befor you can see your Feed");
    }
    let tweets: Tweet[] = [];
    async function timeline() {
        const res = await fetch("http://localhost:8000/user/timeline", {
            method: "GET",
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            }
        });
        if (res.ok) {
            tweets = await res.json();
            console.log(tweets);
        } else {
            throw new Error(
                (await res.json()).message
            )
        }
    }
</script>


<main>
    <h1>Leon ist toll</h1>
    <SendTweet on:tweet={(event) => {tweets = [event.detail.tweet, ...tweets]}} />
{#await timeline()}
    <h3>Loading timeline...</h3>
{:then} 
    <TweetStream {tweets}/>
{/await}
</main>

<style>
</style>
