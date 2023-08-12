<script lang="ts">
	import { projectKey, type ProjectContext, CommandOptionType, CommandOption } from "$lib/editor/project";
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

    function addCommandOption(e: MouseEvent) {
        PROJECT.update((p) => {
            if (p.currentlyEditing?.command == undefined) return p;

            p.currentlyEditing.command.addOption(new CommandOption(
                "new-option",
                "New option",
                CommandOptionType.String
            ));

            return p;
        });
    }
    function removeOption(option: CommandOption) {
        PROJECT.update((p) => {
            if (p.currentlyEditing?.command == undefined) return p;

            p.currentlyEditing.command.removeOption(option);

            return p;
        });
    }

    function updateOption(e: Event, option: CommandOption) {
        PROJECT.update((p) => {
            if (p.currentlyEditing?.command == undefined) return p;

            const target_name = (e.target as HTMLInputElement).name;
            const new_option = option.clone();

            if (target_name === 'props-command-option-name') {
                new_option.name = (e.target as HTMLInputElement).value;
            } else if (target_name === 'props-command-option-desc') {
                new_option.description = (e.target as HTMLInputElement).value;
            } else if (target_name === 'props-command-option-type') {
                const type = {
                    0: CommandOptionType.String,
                    1: CommandOptionType.User,
                    2: CommandOptionType.Channel
                }[(e.target as HTMLInputElement).value] ?? CommandOptionType.String;
                new_option.type = type;
            } else if (target_name === 'props-command-option-required') {
                new_option.required = (e.target as HTMLInputElement).checked;
            }

            p.currentlyEditing.command.updateOption(option, new_option);

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

            <!-- Command options -->
            <div>
                <label for="props-command-options">Options</label>
                {#each $PROJECT.currentlyEditing.command.options as cmdOption (cmdOption.name)}
                    <div class="props-command-option">
                        <!-- Name, description, type -->
                        <div>
                            <label for="props-command-option-name">Name</label>
                            <input name="props-command-option-name" type="text" on:change={e => updateOption(e, cmdOption)} value={cmdOption.name} />
                        </div>
                        <div>
                            <label for="props-command-option-desc">Description</label>
                            <input 
                                name="props-command-option-desc"
                                type="text" 
                                on:change={e => updateOption(e, cmdOption)}
                                value={cmdOption.description}/>
                        </div>
                        <div class="props-center-v">
                            <input name="props-command-option-required" type="checkbox" checked={cmdOption.required} on:change={e => updateOption(e, cmdOption)} />
                            <label for="props-command-option-required">Required</label>
                        </div>
                        <div>
                            <label for="props-command-option-type">Type</label>
                            <select name="props-command-option-type" value={cmdOption.type} on:change={e => updateOption(e, cmdOption)}>
                                <option value={CommandOptionType.String}>String</option>
                                <option value={CommandOptionType.User}>User</option>
                                <option value={CommandOptionType.Channel}>Channel</option>
                            </select>
                        </div>
                        <!-- Remove button -->
                        <input class="props-command-option-remove" type="button" value="Remove" on:click={() => removeOption(cmdOption)} />
                    </div>
                {/each}
                <input class="props-command-options" type="button" value="Add" on:click={addCommandOption} />
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

    .properties input[type="button"] {
        background-color: #1f1f1f;
        border-radius: 10px;
        border: none;
        padding: .5rem;
        color: var(--white);
        width: calc(100% - 2rem);
        outline: none;
        cursor: pointer;
    }

    .properties input[type="button"]:hover {
        background-color: #2f2f2f;
    }

    .properties label {
        font-size: x-small;
        color: #a0a0a0;
        padding-left: .2rem;
        padding-bottom: 0.2rem;
        height: fit-content;
    }

    .error input {
        outline: 1px solid #ed513e;
    }

    .error > p {
        color: #ed513e;
        font-size: x-small;
        padding: 0.2rem;
    }

    .props-command-option {
        display: flex;
        flex-direction: column;
        padding: 0.5rem;
        background-color: #2f2f2f;
        border-radius: 10px;
        margin-bottom: 0.5rem;
        width: calc(100% - 3rem);
    }

    .props-command-option input:not([type="checkbox"]) {
        min-width: calc(100% - 1rem);
    }

    .props-command-option input[type="checkbox"] {
        appearance: none;
        width: 1rem;
        height: 1rem;
        border-radius: 5px;
        border: 1px solid #1f1f1f;
        cursor: pointer;
    }

    .props-command-option input[type="checkbox"]:checked {
        background-color: #ed513e;
    }

    .props-command-option select {
        min-width: calc(100%);
        background-color: #1f1f1f;
        border-radius: 10px;
        padding: .5rem;
        color: var(--white);
        outline: none;
        border: none;
        cursor: pointer;
    }

    .props-command-option select:hover {
        background-color: #0f0f0f;
    }

    .props-command-option-remove {
        margin-top: 0.5rem;
        min-width: 100% !important;
        background-color: #ed523e !important;
    }

    .props-command-option-remove:hover {
        background-color: #fe634f !important;
    }

    .props-center-v {
        margin-top: 0.5rem;
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .props-center-v label {
        padding-left: 0;
        padding-bottom: 1px;
    }
</style>