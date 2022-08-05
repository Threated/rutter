<script lang="ts">
    import type { Tweet, User } from "src/types";
    import RetweetIcon from "../assets/ion-icons/retweet.svg";
    import LikesIcon from "../assets/ion-icons/likes.svg";
    import ShareIcon from "../assets/ion-icons/share.svg";
    import AnswersIcon from "../assets/ion-icons/answers.svg";
    import { auth_fetch } from "../util";
    import { getContext } from "svelte";
    import Answer from "./Answer.svelte";
    import { navigate } from "svelte-navigator";
    import { viewedTweet, viewedUser } from "../store";
    import { hoverUser } from "../util";
    export let tweet: Tweet;
    export let iconWidth = "1.7rem";
    export let padding = "4rem";
    const formatDate = (date: number) => {
        let seconds = (new Date().getTime() - new Date(date).getTime()) / 1000;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        if (seconds < 60) {
            return Math.floor(seconds) + "s";
        } else if (minutes < 60) {
            return Math.floor(minutes) + "m";
        } else if (hours < 24) {
            return Math.floor(hours) + "h";
        } else if (days < 30) {
            return Math.floor(days) + "d";
        } else {
            return new Date(date).toLocaleDateString();
        }
    };
    const like = async () => {
        let res = await auth_fetch(
            `http://localhost:8000/user/like?id=${tweet.id}`,
            {
                method: "POST",
            }
        );
        if (res.ok) {
            let json = await res.json();
            tweet.liked = json.success;
            if (json.success) {
                tweet.likes++;
            }
        }
    };
    const unlike = async () => {
        let res = await auth_fetch(
            `http://localhost:8000/user/unlike?id=${tweet.id}`,
            {
                method: "POST",
            }
        );
        if (res.ok) {
            let json = await res.json();
            tweet.liked = !json.success;
            if (json.success) {
                tweet.likes--;
            }
        }
    };
    const retweet = async () => {
        let res = await auth_fetch(
            `http://localhost:8000/user/retweet?id=${tweet.id}`,
            {
                method: "POST",
            }
        );
        if (res.ok) {
            let json = await res.json();
            console.log(json);
            tweet.retweeted = json.success;
        }
    };
    const unretweet = async () => {
        let res = await auth_fetch(
            `http://localhost:8000/user/unretweet?id=${tweet.id}`,
            {
                method: "POST",
            }
        );
        if (res.ok) {
            let json = await res.json();
            console.log(json);
            tweet.retweeted = !json.success;
        }
    };
    const visitUser = (user: User) => {
        viewedUser.set(user);
        navigate(`/u/${user.name}`);
    };
    const { open } = getContext("answer");
</script>

<article
    class="tweet"
    style="--padding: {padding};"
    on:click={() => {
        viewedTweet.set(tweet);
        navigate(`/t/${tweet.id}`);
    }}
>
    {#if tweet.retweet_by}
        <div class="retweetby">
            <RetweetIcon width="1rem" />
            <span
                class="gray link"
                on:click={() => visitUser(tweet.retweet_by)}
                use:hoverUser={tweet.retweet_by}
            >
                Retweeted by {tweet.retweet_by.name}
            </span>
        </div>
    {/if}
    <div class="head" on:click|stopPropagation={() => visitUser(tweet.author)}>
        <span class="link" use:hoverUser={tweet.author}>
            <h3>{tweet.author.name}</h3>
            <span class="gray">
                {"@" + tweet.author.name + " · " + formatDate(tweet.published)}
            </span>
        </span>
    </div>
    {#if tweet.answer_to}
        <div>
            <span class="gray">
                Answer to
                <span
                    class="link blue"
                    on:click|stopPropagation={() => visitUser(tweet.answer_to)}
                    use:hoverUser={tweet.answer_to}
                >
                    @{tweet.answer_to.name}
                </span>
            </span>
        </div>
    {/if}
    <p>{tweet.content}</p>
    <div class="icons">
        <div
            class="icon answer"
            on:click|stopPropagation={() => open(Answer, { tweet })}
        >
            <AnswersIcon width={iconWidth} />
            {tweet.replies > 0 ? tweet.replies : ""}
        </div>
        <div
            class="icon retweet"
            class:retweeted={tweet.retweeted}
            on:click|stopPropagation={tweet.retweeted ? unretweet : retweet}
        >
            <RetweetIcon width={iconWidth} />
        </div>
        <div
            class="icon likes"
            class:liked={tweet.liked}
            on:click|stopPropagation={tweet.liked ? unlike : like}
        >
            <LikesIcon width={iconWidth} />
            {tweet.likes > 0 ? tweet.likes : ""}
        </div>
        <div class="icon share">
            <ShareIcon width={iconWidth} />
        </div>
    </div>
</article>

<style>
    .retweetby {
        display: flex;
    }
    .retweetby span:hover {
        text-decoration: underline;
    }
    .tweet {
        padding: 0.8rem var(--padding);
        border-bottom: 1px solid gray;
    }
    .tweet:hover {
        background-color: rgba(91, 112, 131, 0.1);
    }
    .head {
        display: flex;
    }
    .gray {
        color: gray;
        margin-left: 3px;
    }
    .blue {
        color: var(--color-blue);
    }
    .link {
        display: inline-flex;
        position: relative;
        justify-content: center;
    }
    .link:hover {
        text-decoration: underline;
    }
    .icons {
        display: flex;
        margin-top: 0.5rem;
        justify-content: space-between;
    }
    .icon {
        display: flex;
        gap: 0.5rem;
        color: gray;
        align-items: center;
    }
    .answer:hover,
    .share:hover {
        color: var(--color-blue);
    }
    .likes {
        fill: transparent;
    }
    .likes:hover {
        color: var(--color-red);
    }
    .retweet:hover {
        color: var(--color-green);
    }
    .retweeted {
        color: var(--color-green);
    }
    .liked {
        fill: var(--color-red);
        color: var(--color-red);
    }
    p {
        white-space: pre-wrap;
        margin: 0;
    }
    h3 {
        margin: 0;
    }
</style>
