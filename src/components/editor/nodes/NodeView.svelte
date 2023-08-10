<script lang="ts">
	import { NodeConnectionType, type ENode } from "$lib/editor/node";
	import { getContext, onDestroy } from "svelte";
	import type { Writable } from "svelte/store";
	import NodeConnector from "./NodeConnector.svelte";
	import { projectKey, type ProjectContext } from "$lib/editor/project";

    export let node: ENode;
    export let currentZoom: number;

    const PROJECT = getContext<ProjectContext>(projectKey);

    let dragging = false;
    const selectedNodes = getContext<Writable<ENode[]>>("selectedNodes");
    let isSelected = false;
    $: offsetCSS = `transform: translate(${node.x}px, ${node.y}px)`;

    const unsubscribeSelectedNodes = selectedNodes.subscribe((nodes) => {
        isSelected = nodes.includes(node);
    });

    onDestroy(() => {
        selectedNodes.update((nodes) => {
            return nodes.filter((n) => n !== node);
        });
        unsubscribeSelectedNodes();
    });

    function startDrag(e: MouseEvent) {
        if (e.button == 0) {
            dragging = true;
            selectNode();
        }
    }
    function stopDrag(e: MouseEvent) {
        if (e.button == 0) {
            dragging = false;
            if ($PROJECT.currentConnection) {
                $PROJECT.currentConnection = null;
            }
        }
    }
    function moveNode(e: MouseEvent) {
        if (e.buttons === 1 && dragging) {
            node.x += e.movementX * (1 / currentZoom);
            node.y += e.movementY * (1 / currentZoom);
            PROJECT.update((p) => p);
        }
    }

    function selectNode() {
        $selectedNodes = [node]
    }
</script>

<div class="node-view" style={offsetCSS} class:nv-selected={isSelected}>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div 
        class="node-view-header"
        on:mousedown={startDrag}
        >
        <!-- If node has __flow_in__ then it should be there -->
        {#if node.type.inputs.__flow_in__}
            <NodeConnector 
                color="white" 
                type={NodeConnectionType.Flow}
                bind:port={node.iPorts["__flow_in__"]} 
                node={node}
                key="__flow_in__"
                isEndPort
                />
        {/if}

        <div class="nvh-info" class:nvh-center={node.type.inputs.__flow_in__ && node.type.outputs.__flow_out__}>
            <p class="nvh-title">{node.type.title}</p>
            <p class="nvh-desc">{node.type.description}</p>
        </div>

        <!-- If node has __flow_out__ then it should be there -->
        {#if node.type.outputs.__flow_out__}
            <NodeConnector 
                color="white" 
                type={NodeConnectionType.Flow}
                bind:port={node.oPorts["__flow_out__"]} 
                node={node}
                key="__flow_out__"
                />
        {/if}
    </div>
    <div class="node-view-body">
        <div class="nvb-inputs">
            {#each Object.entries(node.type.inputs) as input}
                {#if input[0] !== "__flow_in__"}
                    <div class="nf-block nf-i">
                        <NodeConnector 
                            type={input[1].type.type} 
                            bind:port={node.iPorts[input[0]]} 
                            node={node}
                            key={input[0]}
                            sType={input[1].type.structType}
                            isEndPort
                            />
                        <!-- TODO: Don't show inputs if connected -->
                        {#if input[1].type.type === NodeConnectionType.Number}
                            <input type="number" placeholder={input[1].name} bind:value={node.inputHardcoded[input[0]]} />
                        {:else if input[1].type.type === NodeConnectionType.Text}
                            <input type="text" placeholder={input[1].name} bind:value={node.inputHardcoded[input[0]]}/>
                        {:else if input[1].type.type === NodeConnectionType.Boolean}
                            <input type="checkbox" bind:checked={node.inputHardcoded[input[0]]}/>
                            <!-- svelte-ignore a11y-label-has-associated-control -->
                            <label>{input[1].name}</label>
                        {:else}
                             <p>{input[1].name}</p>
                        {/if}
                    </div>
                {/if}
            {/each}
        </div>
        <div class="nvb-outputs">
            {#each Object.entries(node.type.outputs) as output}
                {#if output[0] !== "__flow_out__"}
                    <div class="nf-block nf-o">
                        <p>{output[1].name}</p>
                        <NodeConnector 
                            type={output[1].type.type} 
                            bind:port={node.oPorts[output[0]]} 
                            node={node}
                            key={output[0]}
                            sType={output[1].type.structType}
                            />
                    </div>
                {/if}
            {/each}
        </div>
    </div>
</div>

<svelte:body on:mousemove={moveNode} on:mouseup={stopDrag} />

<style>
    .node-view {
        display: flex;
        flex-direction: column;
        background-color: #1f1f1f;
        border: 2px solid #1f1f1f;
        font-size: small;
        border-radius: 10px;
        overflow: hidden;
        position: absolute;
        min-width: 135px;
    }

    .node-view-header {
        background-color: #0f0f0f;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        padding: .35em;
        cursor: pointer;
        box-shadow: 0px 0px 5px 0px rgba(0, 0, 0, 0.75);
    }

    .nvb-inputs {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        flex-basis: 50%;
    }

    .nvb-outputs {
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        width: 100%;
        height: 100%;
        flex-basis: 50%;
    }
    .nf-block {
        display: flex;
        flex-direction: row;
        align-items: center;
        padding: 0.3em;
        padding-top: 0.5em;
    }

    .nf-i {
        justify-content: flex-start;
        height: 1em;
    }

    .nf-i > input {
        background-color: #0f0f0f;
        border: none;
        padding: 0.5em;
        padding-left: 0.8em;
        border-radius: 10px;
        color: #c0c0c0;
        font-weight: 500;
        font-size: xx-small;
        text-align: center;
        width: 4rem;
    }

    .nf-i > input[type="checkbox"] {
        appearance: none;
        width: 12px;
        height: 12px;
        border-radius: 5px;
        border: 2px solid #0f0f0f;
        background-color: #0f0f0f;
        padding: 0;
        margin-right: 0.5em;
        outline: 1px solid #3f3f3f;
    }

    .nf-i > input[type="checkbox"]:checked {
        background-color: #3f3f3f;
    }

    .nf-i > label {
        color: #c0c0c0;
        font-weight: 500;
        font-size: xx-small;
    }

    .nf-i > input[type="text"] {
        width: 6rem;
        text-align: start;
    }

    .nf-i > input:focus {
        outline: 1px solid #3f3f3f;
    }

    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }
    
    .nf-o {
        justify-content: flex-end;
        height: 1em;
    }

    .nf-block > p {
        color: #c0c0c0;
        font-weight: 500;
        font-size: xx-small;
    }

    .node-view-body {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        padding: 0.1em;
        padding-bottom: 0.3em;
    }

    .nvh-info {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
    }

    .nvh-center {
        justify-content: center !important;
        align-items: center !important;
        text-align: center;
    }

    .nvh-title {
        color: var(--white);
        font-weight: 700;
        font-size: x-small;
    }

    .nvh-desc {
        color: #7e7e7e;
        font-weight: 500;
        font-size: 0.1em;
        word-wrap: break-word;
        margin-top: 2px;
    }

    .nv-selected {
        border: 2px dashed #7f7f7f;
    }
</style>
