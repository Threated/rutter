<script>
    import { isAuthenticated } from "../store";
    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();
    $: if (!$isAuthenticated) {
        // Replace with cool shit https://svelte.dev/repl/033e824fad0a4e34907666e7196caec4?version=3.48.0
        navigate("/login");
        alert("Login befor you can see your Feed");
    }
</script>


<main>
    <h1>Vite + Svelte + Leon stinkt</h1>
    <h1>Vite + Svelte + Leon stinkt</h1><h1>Vite + Svelte + Leon stinkt</h1><h1>Vite + Svelte + Leon stinkt</h1><h1>Vite + Svelte + Leon stinkt</h1><h1>Vite + Svelte + Leon stinkt</h1><h1>Vite + Svelte + Leon stinkt</h1><h1>Vite + Svelte + Leon stinkt</h1>
</main>

<style>
    .logo {
        height: 6em;
        padding: 1.5em;
        will-change: filter;
    }
    .logo:hover {
        filter: drop-shadow(0 0 2em #646cffaa);
    }
    .logo.svelte:hover {
        filter: drop-shadow(0 0 2em #ff3e00aa);
    }
</style>
