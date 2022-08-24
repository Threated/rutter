<script lang="ts">
    import { auth_fetch } from "../util";
    import { viewedTweet } from "../store";
    import Tweet from "./Tweet.svelte";
    import type { Tweet as TweetT } from "../types";
    import { useParams } from "svelte-navigator";
    const params = useParams();
    let tweet = viewedTweet;
    let tweets: TweetT[] = [];
    let notFound = false;
    const userReplies = async () => {
        const res = await auth_fetch(
            `http://localhost:8000/user/answers/?id=${$params.id}`,
            {
                method: "GET",
            }
        );
        if (res.ok) {
            tweets = await res.json();
            // console.log(tweets);
        } else {
            throw new Error((await res.json()).message);
        }
    };
    const getTweet = async (id: string) => {
        let res = await auth_fetch(`http://localhost:8000/user/tweet/?id=${id}`, {
            method: "GET",
        });
        if (res.ok) {
            tweet.set(await res.json());
        } else {
            notFound = true;
        }
    };
    if (!$tweet) {
        tweet.set({
            author: { name: "...", follower: "...", follows: "..." },
            content: "",
            published: 0,
            id: "",
            answer_to: null,
            retweet_by: null,
            likes: 0,
            replies: 0,
            liked: false,
            retweeted: false,
        });
        getTweet($params.id);
    }
</script>

<div class="currentTweet">
    {#if notFound}
        <h1>Can not find with id {$tweet.id}</h1>
    {:else}
        <Tweet tweet={$tweet} padding="2rem" />
    {/if}
</div>
<hr />
{#await userReplies()}
    <h3>Loading tweets...</h3>
{:then}
    {#each tweets as tweet}
        <Tweet {tweet} />
    {/each}
{/await}

<style>
    hr {
        margin: 0;
        background: gray;
    }
</style>
