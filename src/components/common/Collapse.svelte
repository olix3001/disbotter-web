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
        background-color: #060606;
        display: flex;
        align-items: center;
        padding: .3em;
        cursor: pointer;
    }
    .header:hover {
        background-color: #0f0f0f;
    }
    .body {
        background-color: #1f1f1f;
    }
</style>