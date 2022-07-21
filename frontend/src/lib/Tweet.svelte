<script lang="ts">
    import type { Tweet } from "src/types";
    import RetweetIcon from "../assets/ion-icons/retweet.svg";
    import LikesIcon from "../assets/ion-icons/likes.svg";
    import ShareIcon from "../assets/ion-icons/share.svg";
    import AnswersIcon from "../assets/ion-icons/answers.svg";
    import { auth_fetch } from "../main";
    export let tweet: Tweet;
    export const iconWidth = "1.7rem";
    const formatDate = (date: number) => {
        let seconds = (new Date().getTime() - new Date(date).getTime()) / 1000;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        if (seconds < 60) {
            return Math.floor(seconds) + "s"
        } else if (minutes < 60) {
            return Math.floor(minutes) + "m"
        } else if (hours < 24) {
            return Math.floor(hours) + "h"
        } else if (days < 30) {
            return Math.floor(days) + "d"
        } else {
            return new Date(date).toLocaleDateString()
        }
    }
    const like = async () => {
        let res = await auth_fetch(`http://localhost:8000/user/like?id=${tweet.id}`, {
            method: "POST"
        });
        if (res.ok) {
            let json = await res.json()
            console.log(json)
            tweet.liked = json.success;
            if (json.success) {
                tweet.likes++;
            }
        }
    }
    const unlike = async () => {
        let res = await auth_fetch(`http://localhost:8000/user/unlike?id=${tweet.id}`, {
            method: "POST"
        });
        if (res.ok) {
            let json = await res.json()
            console.log(json)
            tweet.liked = !json.success
            if (json.success) {
                tweet.likes--;
            }
        }
    }
    const retweet = async () => {
        let res = await auth_fetch(`http://localhost:8000/user/retweet?id=${tweet.id}`, {
            method: "POST"
        });
        if (res.ok) {
            let json = await res.json()
            console.log(json)
            tweet.retweeted = json.success
        }
    }
    const unretweet = async () => {
        let res = await auth_fetch(`http://localhost:8000/user/unretweet?id=${tweet.id}`, {
            method: "POST"
        });
        if (res.ok) {
            let json = await res.json()
            console.log(json)
            tweet.retweeted = !json.success
        }
    }
</script>

<div class="tweet">
    <div class="head">
        <h3>{tweet.author.name}</h3>
        <span>
            {"@" + tweet.author.name + " · " + formatDate(tweet.published)}
        </span>
    </div>
    <p>{tweet.content}</p>
    <div class="icons">
        <div class="icon answer">
            <AnswersIcon width={iconWidth} />
            {tweet.replies > 0 ? tweet.replies : ""}
        </div>
        <div class="icon retweet" class:retweeted="{tweet.retweeted}" on:click={tweet.retweeted ? unretweet : retweet}>
            <RetweetIcon width={iconWidth} />
        </div>
        <div class="icon likes" class:liked="{tweet.liked}" on:click={tweet.liked ? unlike : like}>
            <LikesIcon width={iconWidth} />
            {tweet.likes}
        </div>
        <div class="icon share">
            <ShareIcon width={iconWidth} />
        </div>
    </div>
</div>

<style>
    .tweet {
        padding: 0.8rem 4rem;
        border-bottom: 1px solid gray;
    }
    .head {
        display: flex;
    }
    span {
        color: gray;
        margin-left: 3px;
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
    .liked {
        fill: rgb(206, 74, 74);
        color: rgb(206, 74, 74);
    }
    .retweeted {
        color: rgb(0, 208, 42);
    }
    .answer:hover,
    .share:hover {
        color: var(--color-blue);
    }
    .likes:hover {
        color: rgb(206, 74, 74);
    }
    .retweet:hover {
        color: rgb(0, 208, 42);
    }
    p {
        white-space: pre-wrap;
        margin: 0;
    }
    h3 {
        margin: 0;
    }
</style>
