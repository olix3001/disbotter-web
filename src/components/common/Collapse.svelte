<script lang="ts">
    export let open = false;

    function toggle() {
        open = !open;
    }
</script>

<div class="collapse">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="header" on:click={toggle}>
        <!-- Collapsed icon -->
        <svg fill="white" viewBox="0 0 10 10">
            {#if open}
                <!-- Triangle -->
                <path d="M 0 0 L 10 0 L 5 10 z" />
            {:else}
                <!-- Triangle Collapsed -->
                <path d="M 0 0 L 10 5 L 0 10 z" />
            {/if}
        </svg>
        <slot name="header" />
    </div>
    {#if open}
        <div class="body">
            <slot />
        </div>
    {/if}
</div>

<style>
    .header > svg {
        min-width: 10px;
        max-width: 10px;
        width: 10px;
        height: 10px;
        margin-right: .5em;
    }

    .header {
        background-color: #1f1f1f;
        display: flex;
        align-items: center;
        padding: .3em;
        cursor: pointer;
    }
    .header:hover {
        background-color: #0f0f0f;
    }
    .body {
        background-color: #0f0f0f;
        box-shadow: inset 0px 0px 5px 0px rgba(0,0,0,0.75);
    }
</style>