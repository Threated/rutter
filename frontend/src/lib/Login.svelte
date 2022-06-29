<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();
    import { isAuthenticated } from "../store";
    let email: string;
    let password: string;
    let loading: boolean;
    let errorMsg: string;
    const handleSubmit = () => {
        let loginFields = { email, password };
        const endpoint = `localhost:8000/auth/login`;
        loading = true;
        fetch(endpoint, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(loginFields),
        })
            .then((response) => {
                console.log(response);
                if (response.status === 200) {
                    isAuthenticated.set(true);
                    navigate("/");
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

<h3>Login</h3>
<form on:submit|preventDefault={handleSubmit}>
    <input
        class="form-field"
        bind:value={email}
        type="email"
        placeholder="Email"
    />
    <input
        class="form-field"
        bind:value={password}
        type="password"
        placeholder="Password"
    />
    <button disabled={loading} class="form-field"> Login </button>
</form>
{#if errorMsg}
    <p class="error">Error ❌ {errorMsg}</p>
{/if}
<p>
    Don't have an account?
    <strong class="link" on:click={() => navigate("/signup")}>Sign up</strong>
</p>

<style>
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        background-color: #1a1a1a;
        cursor: pointer;
        transition: border-color 0.25s;
    }
    button:hover {
        border-color: #646cff;
    }
    button:focus,
    button:focus-visible {
        outline: 4px auto -webkit-focus-ring-color;
    }
</style>
