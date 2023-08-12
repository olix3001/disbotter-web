<script lang="ts">
	import type { NodeType } from "$lib/editor/node";
	import { type ProjectContext, projectKey } from "$lib/editor/project";
	import { getContext, onDestroy, createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";

    let position: { x: number; y: number } = { x: 0, y: 0 };
    $: positionCSS = `transform: translate(${position.x}px, ${position.y}px)`;
    let open = false;
    let inputField: HTMLInputElement;

    let search = "";
    const dispatch = createEventDispatcher<{ nodeselected: {
        node: NodeType;
        position: { x: number; y: number };
    } }>();

    const PROJECT = getContext<ProjectContext>(projectKey);
    let categorizedNodes: { [key: string]: NodeType[] } = {};

    $: filteredSearch = Object.fromEntries(
        Object.entries(categorizedNodes).map(([category, nodes]) => (
            [
                category, 
                nodes.filter(node => {
                    return node.title.toLowerCase().includes(search.toLowerCase()) || 
                        node.description.toLowerCase().includes(search.toLowerCase());
                })
            ]
        ))
    );

    const unsubscribeProject = PROJECT.subscribe((project) => {
        categorizedNodes = {};
        project.getCurrentFlow()?.availableNodes.forEach(an => {
            if (!categorizedNodes[an.category]) categorizedNodes[an.category] = [];
            categorizedNodes[an.category].push(an);
        });
    });

    onDestroy(() => {
        unsubscribeProject(); 
    })
    
    export const openContext = (new_position: { x: number; y: number }) => {
        open = true;
        position = new_position;
        if (inputField) inputField.focus();
    };

    export const closeContext = () => {
        open = false;
    };

    let contextMenu: HTMLDivElement;
    function closeContextCheck(e: MouseEvent) {
        // Check if the click was outside the context
        if (open && e.target !== contextMenu && !contextMenu.contains(e.target as Node)) {
            closeContext();
        }
    }

</script>

{#if open}
    <div 
        class="editor-context-search"
        bind:this={contextMenu}
        style={positionCSS} 
        transition:fade={{
            duration: 150
        }}
        on:introend={() => inputField.focus()}
        on:outroend={() => { search = ""; }}
        >
        <!-- Search bar -->
        <div class="editor-context-search-bar">
            <input 
                type="text"
                placeholder="Search..."
                bind:this={inputField}
                bind:value={search}
                />
        </div>

        <!-- Nodes with categories -->
        <div class="editor-context-categories">
            {#each Object.keys(filteredSearch) as category}
                {#if filteredSearch[category].length !== 0}
                    <div class="ec-category">
                        <div class="ec-category-title">{category}</div>
                        <div class="ec-category-body">
                            <!-- svelte-ignore a11y-no-static-element-interactions -->
                            <!-- svelte-ignore a11y-click-events-have-key-events -->
                            {#each filteredSearch[category] as node}
                                <div class="ec-node" on:click={() => {
                                    dispatch('nodeselected', {
                                        node,
                                        position
                                    });
                                    closeContext();
                                }}>
                                    <div class="ec-node-title">{node.title}</div>
                                    <div class="ec-node-description">{node.description}</div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}
            {/each}
        </div>
    </div>
{/if}

<svelte:body on:mousedown={closeContextCheck} />

<style>
    .editor-context-search {
        position: absolute;
        display: flex;
        transition: transform 120ms ease-in-out;
        background-color: #181818;
        min-width: 250px;
        max-width: 350px;
        box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
        z-index: 9999;
        border-radius: 5px;
        border: 1px solid #323232;
        flex-direction: column;
        align-items: flex-start;
    }

    .editor-context-search-bar {
        width: calc(100% - 1rem);
        padding: 0.5rem;
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: center;
    }

    .editor-context-search-bar input {
        width: 100%;
        background-color: #323232;
        border: none;
        padding: 0.5rem;
        border-radius: 5px;
        color: var(--white);
        outline: none;
    }

    .editor-context-categories {
        width: 100%;
        overflow-y: auto;
        max-height: 20rem;
    }

    .editor-context-categories::-webkit-scrollbar {
        width: 9px;
    }

    .editor-context-categories::-webkit-scrollbar-thumb {
        background-color: #323232;
        border-radius: 4.5px;
        border: 2px #181818 solid;
        background-clip: padding-box;
    }

    .ec-category {
        width: calc(100% - 1rem);
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .ec-category-title {
        color: var(--white);
        font-weight: 700;
        font-size: medium;
        margin-bottom: 0.5rem;
    }

    .ec-category-body {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .ec-node {
        width: calc(100% - 1rem);
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        border-radius: 5px;
        background-color: #323232;
        margin-bottom: 0.2rem;
        cursor: pointer;
    }

    .ec-node:hover {
        background-color: #424242;
    }

    .ec-node-title {
        color: var(--white);
        font-weight: 700;
        font-size: small;
        margin-bottom: 2px;
    }

    .ec-node-description {
        color: #9f9f9f;
        font-weight: 400;
        font-size: x-small;
    }
</style>