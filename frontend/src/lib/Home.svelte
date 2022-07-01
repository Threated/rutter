<script>
    import { isAuthenticated } from "../store";
    import { useNavigate } from "svelte-navigator";
    import SendTweet from "./SendTweet.svelte";
    const navigate = useNavigate();
    $: if (!$isAuthenticated) {
        // Replace with cool shit https://svelte.dev/repl/033e824fad0a4e34907666e7196caec4?version=3.48.0
        navigate("/login");
        alert("Login befor you can see your Feed");
    }
</script>


<main>
    <h1>Leon ist toll</h1>
    <SendTweet />
</main>

<style>
</style>
