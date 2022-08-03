<script lang="ts">
    import { getContext } from "svelte";
    import { auth_fetch } from "../main";
    import type { Tweet as TweetT } from "../types";
    import SendTweet from "./SendTweet.svelte";
    import Tweet from "./Tweet.svelte";
    let content;
    export let tweet: TweetT;
    const fetch_answer = () => auth_fetch("http://localhost:8000/user/answer", {
        method: "POST",
        body: JSON.stringify({content, id: tweet.id})
    })
    const {close} = getContext("answer");
    // https://svelte.dev/repl/033e824fad0a4e34907666e7196caec4?version=3.4.1
</script>

<Tweet {tweet} />
<div class="send">
    <SendTweet placeholder="Answer here..." bind:content on:tweet={close} sender={fetch_answer} />
</div>

<style>
.send {
    padding: 1rem 4rem;
}
</style>
