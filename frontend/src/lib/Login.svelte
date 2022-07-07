<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();
    import { isAuthenticated } from "../store";
    import Input from "./Input.svelte";
    let name: string;
    let password: string;
    let loading: boolean;
    let errorMsg: string;
    const handleSubmit = () => {
        let loginFields = { name, password };
        loading = true;
        fetch("http://localhost:8000/auth/login", {
            method: "POST",
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(loginFields),
        })
            .then((response) => {
                if (response.ok) {
                    isAuthenticated.set(true);
                    navigate("/");
                } else {
                    response.json().then((data) => {
                        errorMsg = data.message;
                    });
                }
            })
            .catch((error) => console.log(error))
            .finally(() => (loading = false));
    };
</script>

<div class="login">
    <h3>Login</h3>
    <form on:submit|preventDefault={handleSubmit}>
        <Input
            bind:value={name}
            label="Username"
            autocomplete="username"
        />
        <Input
            bind:value={password}
            type="password"
            label="Password"
            autocomplete="current-password"
        />
        <button disabled={loading} class="form-field"> Login </button>
    </form>
    {#if errorMsg}
    <p class="error">Error ❌ {errorMsg}</p>
    {/if}
    <p>
        Don't have an account?
        <button class="form-field" on:click={() => navigate("/signup")}> Sign-up </button>
    </p>
</div>

<style>
    .login {
        display: flex;
        align-content: center;
        flex-direction: column;
        flex-wrap: wrap;
        align-items: flex-start;
    }

    form {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 1em;
    }

    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        background-color: #4b4848;
        cursor: pointer;
        transition: border-color 0.25s;
    }
    button:hover {
        color: #646cff;
        border-color: #646cff;
    }
    button:focus,
    button:focus-visible {
        outline: 4px auto -webkit-focus-ring-color;
    }
</style>
