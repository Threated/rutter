<script lang="ts">
    let content: string;
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
                if (response.status === 201) {
                    response.json().then((data) => alert(data.message));
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
