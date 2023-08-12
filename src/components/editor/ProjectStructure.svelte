<script lang="ts">
	import { getContext } from "svelte";
	import Collapse from "../common/Collapse.svelte";
	import { Command, projectKey, type ProjectContext } from "$lib/editor/project";

    const PROJECT = getContext<ProjectContext>(projectKey);

    function addCommand() {
        PROJECT.update(p => {
            p.addCommand(new Command('new-command', 'Do not leave empty description.'));
            return p;
        });
    }

    function setEditing(command: Command) {
        PROJECT.update(p => {
            p.setEditing(command);
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
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        {#each $PROJECT.commands as command (command.uid)}
             <!-- svelte-ignore a11y-click-events-have-key-events -->
             <div class="element" class:selected={$PROJECT.isEditing(command)} on:click={() => setEditing(command)}>
                <p>{command.name}</p>
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
        background-color: #1f1f1f;
        border: none;
        color: var(--white);
        padding: .3em;
        cursor: pointer;
    }

    .header button:hover {
        background-color: #0f0f0f;
    }

    .element {
        cursor: pointer;
        margin-bottom: .2em;
    }

    .element:hover {
        background-color: #0f0f0f;
    }

    .selected {
        color: #4d86ff;
        padding-left: .25rem;
    }

</style>