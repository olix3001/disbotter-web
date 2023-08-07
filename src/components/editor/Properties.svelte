<script lang="ts">
	import { projectKey, type ProjectContext } from "$lib/editor/project";
	import { getContext } from "svelte";

    const PROJECT = getContext<ProjectContext>(projectKey);
</script>

{#if $PROJECT.currentlyEditing === null}
    <div class="not-editing">
        <p>Select something to edit properties.</p>
    </div>
{:else}
    <!-- Properties of a selected command -->
    {#if $PROJECT.currentlyEditing.command != undefined}
        <div>
            <p>Command Name: {$PROJECT.currentlyEditing.command.name}</p>
            <p>Command Description: {$PROJECT.currentlyEditing.command.description}</p>
        </div>
    {/if}
{/if}

<style>
    .not-editing {
        display: flex;
        flex-direction: column;
        padding-top: 2rem;
        align-items: center;
        height: 50%;
    }
</style>