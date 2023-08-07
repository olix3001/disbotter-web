<script lang="ts">
	import { getContext } from "svelte";
	import Collapse from "../common/Collapse.svelte";
	import { Command, projectKey, type ProjectContext } from "$lib/editor/project";

    const PROJECT = getContext<ProjectContext>(projectKey);

    function addCommand() {
        PROJECT.update(p => {
            p.commands.push(new Command('New command', ''));
            return p;
        });
    }
</script>

<Collapse open={true}>
    <div slot="header" class="header">
        <p>Commands</p>
        <button on:click|stopPropagation={addCommand}>New</button>
    </div>
    <div>
        {#each $PROJECT.commands as command (command.uid)}
             <div>
                <p>{command.name}</p>
                <p>{command.description}</p>
             </div>
        {:else}
            <p>No commands yet.</p>
        {/each}
    </div>
</Collapse>
<Collapse open={true}>
    <div slot="header">
        <p>Events</p>
    </div>
    <div>
        <p>Events go here...</p>
    </div>
</Collapse>

<style>
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;
    }

    .header button {
        background-color: #060606;
        border: none;
        color: var(--white);
        padding: .3em;
        cursor: pointer;
    }
</style>