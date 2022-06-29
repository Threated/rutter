<script lang="ts">
    import { Route, Router } from "svelte-navigator";
    import Home from "./lib/Home.svelte";
    import Login from "./lib/Login.svelte";
    import Navbar from "./lib/Navbar.svelte";
    import Register from "./lib/Register.svelte";
</script>

<Router>
    <main>
        <div><Navbar/></div>
        <div class="main">
            <Route path="/">
                <Home />
            </Route>
            <Route path="login" component={Login} />
            <Route path="register" component={Register} />
        </div>
    </main>
</Router>

<style>
    main {
        display: grid;
        /* grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); */
        grid-template-columns: 1fr 2fr 1fr;
        /* grid-template-areas: 
            ". head ."
            "side main ."
            ". footer ."; */
    }
    .navbar {
        /* grid-area: side; */
        border: 3px solid #00f;
    }
    .main {
        /* display: flex; */
        /* height: 100vh; */
        /* grid-area: main; */
        border: 3px solid #00f;
    }
</style>
