<script lang="ts">
	import { projectKey, type ProjectContext } from "$lib/editor/project";
	import { getContext, onDestroy } from "svelte";

    const PROJECT = getContext<ProjectContext>(projectKey);

    let commandName = $PROJECT.currentlyEditing?.command?.name ?? "New command";
    let commandDesc = $PROJECT.currentlyEditing?.command?.description ?? "";

    const unsubscribe = PROJECT.subscribe((p) => {
        commandName = p.currentlyEditing?.command?.name ?? "New command";
        commandDesc = p.currentlyEditing?.command?.description ?? "";
    });

    onDestroy(() => {
        unsubscribe();
    });

    const errors: { [key: string]: { isError: boolean, error: string } } = {
        "command-name": {
            isError: false,
            error: ""
        },
        "command-desc": {
            isError: false,
            error: ""
        }
    };

    function updateCommandName(e: Event) {
        PROJECT.update((p) => {
            if (p.currentlyEditing?.command == undefined) return p;

            const name = (e.target as HTMLInputElement).value;
            if (name.length == 0) {
                errors["command-name"].isError = true;
                errors["command-name"].error = "Command name cannot be empty.";
                p.currentlyEditing.command.name = "new-command";
            } else if (!name.match(/[a-z][a-z0-9-]+$/)) {
                errors["command-name"].isError = true;
                errors["command-name"].error = "Command name can only contain lowercase letters, numbers and dashes."; 
            } else {
                errors["command-name"].isError = false;
                errors["command-name"].error = "";
                p.currentlyEditing.command.name = (e.target as HTMLInputElement).value;
            }

            return p;
        });
    }

    function updateCommandDesc(e: Event) {
        PROJECT.update((p) => {
            if (p.currentlyEditing?.command == undefined) return p;

            p.currentlyEditing.command.description = (e.target as HTMLInputElement).value;

            return p;
        });
    }
</script>

{#if $PROJECT.currentlyEditing === null}
    <div class="not-editing">
        <p>Select something to edit properties.</p>
    </div>
{:else}
    <!-- Properties of a selected command -->
    {#if $PROJECT.currentlyEditing.command != undefined}
        <div class="properties">
            <div class:error={errors['command-name'].isError}>
                <label for="props-command-name">Command Name</label>
                <input name="props-command-name" type="text" on:input={updateCommandName} bind:value={commandName} />
                <p>{errors['command-name'].error}</p>
            </div>
            <div class:error={errors['command-desc'].isError}>
                <label for="props-command-desc">description</label>
                <input name="props-command-desc" type="text" on:input={updateCommandDesc} bind:value={commandDesc} />
                <p>{errors['command-desc'].error}</p>
            </div>
        </div>
    {/if}
{/if}

<style>
    .properties {
        display: flex;
        flex-direction: column;
        padding-top: 2rem;
        padding: 1rem;
        align-items: flex-start;
        width: 100%;
    }

    .not-editing {
        display: flex;
        flex-direction: column;
        padding-top: 2rem;
        align-items: center;
        height: 50%;
    }

    .properties > div {
        display: flex;
        flex-direction: column;
        width: 100%;
        margin-bottom: .5rem;
    }

    .properties input {
        background-color: #1f1f1f;
        border-radius: 10px;
        border: none;
        padding: .5rem;
        color: var(--white);
        width: calc(100% - 3rem);
        outline: none;
    }

    .properties label {
        font-size: x-small;
        color: #a0a0a0;
        padding-left: .2rem;
        padding-bottom: 0.2rem;
    }

    .error input {
        outline: 1px solid #ed513e;
    }

    .error > p {
        color: #ed513e;
        font-size: x-small;
        padding: 0.2rem;
    }

</style>