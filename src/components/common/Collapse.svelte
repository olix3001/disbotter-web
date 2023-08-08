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
        <slot name="header" />
    </div>
    {#if open}
        <div class="body">
            <slot />
        </div>
    {/if}
</div>

<style>
    .header {
        background-color: #070808;
        display: flex;
        align-items: center;
        padding: .3em;
        cursor: pointer;
    }
    .header:hover {
        background-color: #030303;
    }
    .body {
        background-color: #0f0f0f;
        box-shadow: inset 0px 0px 5px 0px rgba(0,0,0,0.75);
    }
</style>