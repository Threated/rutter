<script lang="ts">
    import { auth_fetch } from "../main";
    import { viewedUser } from "../store";
    import Tweet from "./Tweet.svelte";
    import type { Tweet as TweetT } from "../types";
    import { useParams } from "svelte-navigator";
    const params = useParams();
    let user = viewedUser;
    let tweets: TweetT[] = [];
    let notFound = false;
    const userTweets = async () => {
        const res = await auth_fetch(
            `http://localhost:8000/user/tweets/${$user.name}`,
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
    const getUser = async (name: string) => {
        let res = await fetch(`http://localhost:8000/user/info/${name}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            }
        });
        if (res.ok) {
            user.set(await res.json());
        } else {
            notFound = true;
        }
    }
    if (!$user) {
        user.set({ name: $params.user, follower: "...", follows: "..." });
        getUser($params.user);
    }
</script>

<div class="user">
    {#if notFound}    
    <h1>Can not find {$user.name}</h1>
    {:else}
    <h3>{$user.name}</h3>
    <span>{"@" + $user.name}</span>
    <div class="info">
        <div>{$user.follows} <span>Follows</span></div>
        <div>{$user.follower} <span>Follower</span></div>
    </div>
    {/if}
</div>
{#await userTweets()}
    <h3>Loading tweets...</h3>
{:then}
    {#each tweets as tweet}
        <Tweet {tweet} />
    {/each}
{/await}

<style>
    span {
        color: gray;
    }
    h3 {
        margin: 0;
    }
    .info {
        display: flex;
        margin-top: 2rem;
        gap: 1rem;
    }
    .user {
        padding: 2rem;
        border-bottom: 1px solid white;
    }
</style>
