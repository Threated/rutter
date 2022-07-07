<script lang="ts">
    import HomeIcon from "../assets/ion-icons/home.svg";
    import SearchIcon from "../assets/ion-icons/search.svg";
    import LoginIcon from "../assets/ion-icons/log-in.svg";
    import LogoutIcon from "../assets/ion-icons/log-out.svg";
    import {isAuthenticated} from "../store";
    import NavButton from "./NavButton.svelte";
    const logout = () => {
        isAuthenticated.set(false);
        fetch("http://localhost:8000/auth/logout", {
            method: "POST",
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            }
        })
    }
</script>

<nav>
    <div>
        <NavButton to="/" icon={HomeIcon}>
            Home
        </NavButton>
        <NavButton to="/search" icon={SearchIcon}>
            Search
        </NavButton>
    {#if $isAuthenticated}
        <NavButton to={logout} icon={LogoutIcon}>
            Logout
        </NavButton>
    {:else}
        <NavButton to="/login" icon={LoginIcon}>
            Login
        </NavButton>
    {/if}
    </div>
</nav>

<style>
    nav {
        display: flex;
        position: sticky;
        top: 0;
        justify-content: right;
        margin-right: 2rem;
    }
    div {
        padding-left: 0.5rem;
        border-left: 2px solid white;
    }
</style>
