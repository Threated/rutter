<script lang="ts">
    import { navigate } from "svelte-navigator";
    import Input from "./Input.svelte";
    let name: string;
    let password: string;
    let loading: boolean;
    let errorMsg: string;
    const handleSubmit = () => {
        let loginFields = { name, password };
        loading = true;
        fetch("http://localhost:8000/auth/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(loginFields),
        })
            .then((response) => {
                if (response.status === 201) {
                    navigate("/login");
                    response.json().then((data) => alert(data.message));
                } else {
                    response.json().then((data) => {
                        console.log(data);
                        errorMsg = data.message;
                    });
                }
            })
            .catch((error) => console.log(error))
            .finally(() => (loading = false));
    };
</script>

<div class="login">
    <h3>Register</h3>
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
            autocomplete="new-password"
        />
        <button disabled={loading} class="form-field"> Sgin-up </button>
    </form>
    {#if errorMsg}
    <p class="error">Error ❌ {errorMsg}</p>
    {/if}
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
        border-radius: 2rem;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
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
