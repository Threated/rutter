<script lang="ts">
    import { setContext } from "svelte";
    import CloseIcon from "../assets/ion-icons/close.svg";
    let component
    let props = {tweet: {author: {}}}
    function close() {
        component = null;
    }
    function open(comp, propertys) {
        component = comp
        props = propertys
    }
    function handlekeydown(event: KeyboardEvent) {
        if (component && event.key === 'Escape') {
            event.preventDefault();
            close();
        }
    }
    
    export let modalName: string;
    setContext(modalName, {open, close})
</script>

<svelte:window on:keydown={handlekeydown} />
{#if component}
<div id="topModal" on:click={close}>
	<div id='modal' on:click|stopPropagation>
		<div class="top">
            <div class="btn" on:click={close}>
                <CloseIcon width=2rem />
            </div>
        </div>
        <div id='modal-content'>
            <svelte:component this={component} {...props} />
		</div>
	</div>
</div>
{/if}
<slot />

<style>
	#topModal {
		z-index: 9999;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(91, 112, 131, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	#modal {
		position: relative;
		border-radius: 6px;
		background: var(--color-background);
        width: 35vw;
		filter: drop-shadow(5px 5px 5px rgb(39, 39, 39));
		padding: 1em;
        margin-bottom: 30vh;
	}
    .top {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
    .btn {
        display: flex;
    }
    .btn:hover {
        background-color: rgba(91, 112, 131, 0.4);
        border-radius: 50%;
    }
	#modal-content {
		max-width: calc(100vw - 20px);
		max-height: calc(100vh - 20px);
		overflow: auto;
	}
</style>
