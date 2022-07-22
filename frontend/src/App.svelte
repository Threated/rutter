<script lang="ts">
    import { Route, Router } from "svelte-navigator";
    import Home from "./lib/Home.svelte";
    import Login from "./lib/Login.svelte";
    import Modal from "./lib/Modal.svelte";
    import Navbar from "./lib/Navbar.svelte";
    import Register from "./lib/Register.svelte";
</script>

<Router>
    <Modal modalName="answer">
        <main>
            <div><Navbar/></div>
            <div class="main">
                <Route path="/">
                    <Home />
                </Route>
                <Route path="login" component={Login} />
                <Route path="signup" component={Register} />
            </div>
        </main>
    </Modal>
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
    .main {
        /* display: flex; */
        /* height: 100vh; */
        /* grid-area: main; */
        border: 1px solid rgb(128, 128, 128);
        border-top: 0;
        border-bottom: 0;
        /* padding: .5rem 2rem; */
        /* padding-top: .5rem; */
    }
</style>
