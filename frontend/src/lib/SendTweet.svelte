<script lang="ts">
import { createEventDispatcher } from "svelte";


    let content: string;
    const dispatch = createEventDispatcher();
    const sendTweet = () => {
        fetch("http://localhost:8000/user/tweet", {
            method: "POST",
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ content })
        }).then((response) => {
            console.log(response);
            if (response.status === 200) {
                response.json().then((tweet) => dispatch("tweet", {tweet}));
                content = "";
            } else {
                response.json().then((data) => {
                    console.log(response.status, data);
                });
            }
        })
    }
</script>

<input type="text" bind:value={content} placeholder="Tweet"/>
<button on:click={sendTweet}>Rutter</button>

<style>
    input {
        border: 0;
    }
</style>
