<script lang="ts">
    export let value: string;
    export let label: string;
    export let autocomplete: string = "off";
    export let type: string = "text";
    const setType = () => {
        return { type: type };
    };
</script>

<label class="input">
    <input
        class="input__field"
        {...setType()}
        placeholder=" "
        {autocomplete}
        bind:value
    />
    <span class="input__label">{label}</span>
</label>

<!-- This design is from https://codepen.io/meodai/pen/rNedxBa -->
<style>
    .input {
        --size-bezel: 0.5rem;
        --size-radius: 1rem;
        position: relative;
    }
    .input__label {
        position: absolute;
        left: 0;
        top: 0;
        padding: calc(var(--size-bezel) * 0.75) calc(var(--size-bezel) * 0.5);
        margin: calc(var(--size-bezel) * 0.75 + 3px)
            calc(var(--size-bezel) * 0.5);
        white-space: nowrap;
        border-radius: var(--size-radius);
        transform: translate(0, 0);
        transform-origin: 0 0;
        background: var(--color-background);
        transition: transform 120ms ease-in;
        font-weight: bold;
        line-height: 1.2;
        user-select: none;
    }
    .input__field {
        box-sizing: border-box;
        display: block;
        width: 100%;
        border: 2px solid var(--color-blue);
        padding: calc(var(--size-bezel) * 1.5) var(--size-bezel);
        color: currentColor;
        background: transparent;
        border-radius: var(--size-radius);
    }
    .input__field:focus + .input__label,
    .input__field:not(:placeholder-shown) + .input__label {
        transform: translate(0.25rem, -65%) scale(0.8);
        /* color: var(--color-blue); */
    }
</style>
