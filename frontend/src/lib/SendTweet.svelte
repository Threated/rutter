<script lang="ts">
    import { auth_fetch } from "../main";
    import { createEventDispatcher } from "svelte";
    let content: string = "";
    const dispatch = createEventDispatcher();
    const sendTweet = () => {
        auth_fetch("http://localhost:8000/user/tweet", {
            method: "POST",
            body: JSON.stringify({ content }),
        }).then((response) => {
        
            console.log(response);
            if (response.status === 200) {
                response.json().then((tweet) => dispatch("tweet", { tweet }));
                content = "";
            } else {
                response.json().then((data) => {
                    console.log(response.status, data);
                });
            }
        });
    };
</script>

<div class="container">
    <div class="input">
        <pre aria-hidden="true">{content + "\n"}</pre>
        <textarea bind:value={content} placeholder="Tweet here..." />
    </div>
    <button on:click={sendTweet}>Rutter</button>
</div>


<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 1rem;
    }
    .input {
        position: relative;
        width: 20rem;
    }
    pre {
        margin: 0;
    }
    pre, textarea {
        background-color: var(--color-background);
        font-family: inherit;
        font-size: 1rem;
        line-height: 1.2;
        box-sizing: border-box;
        overflow: hidden;
        padding: .3rem;
    }
    textarea {
        position: absolute;
        resize: none;
        top: 0;
        border: 0;
        /* border-radius: .5rem; */
        width: 100%;
		height: 100%;
    }
    button {
        border-radius: 2rem;
        border: 1px solid transparent;
        padding: 0.5em 1em;
        font-size: 1.2em;
        font-weight: 500;
        font-family: inherit;
        background-color: var(--color-blue);
        transition: border-color 0.25s;
    }
    button:hover {
        filter: brightness(85%);
    }
    button:focus,
    button:focus-visible {
        outline: 4px auto -webkit-focus-ring-color;
    }
</style>
