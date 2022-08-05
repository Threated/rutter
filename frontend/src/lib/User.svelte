<script lang="ts">
    import { auth_fetch } from "../util";
    import { isAuthenticated, loggedinUsername, viewedUser } from "../store";
    import Tweet from "./Tweet.svelte";
    import type { Tweet as TweetT } from "../types";
    import { useParams } from "svelte-navigator";
    const params = useParams();
    let user = viewedUser;
    let tweets: TweetT[] = [];
    let notFound = false;
    let isFollower = false;
    const follow = async () => {
        let res = await auth_fetch(
            `http://localhost:8000/user/follow/${$user.name}`,
            {
                method: "POST",
            }
        );
        if (res.ok) {
            isFollower = true
        }
    };
    const unfollow = async () => {
        let res = await auth_fetch(
            `http://localhost:8000/user/unfollow/${$user.name}`,
            {
                method: "POST",
            }
        );
        if (res.ok) {
            isFollower = false
        }
    };
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
            res = await res.json();
            user.set(res[0]);
            isFollower = res[1];
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
    <div class="head">
        <div>
            <h3>{$user.name}</h3>
            <span>{"@" + $user.name}</span>
        </div>
        {#if $isAuthenticated && $loggedinUsername !== $user.name}
            <button on:click={isFollower? unfollow : follow}>{isFollower? "Unfollow" : "Follow"}</button>
        {/if}
    </div>
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
    .head {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    button {
        font-size: large;
        border: 1px solid var(--color-white);
        background: none;
        border-radius: 25px;
        padding: .5rem 1rem;
    }
    button:hover {
        background-color: rgba(91, 112, 131, 0.2);
    }
</style>
