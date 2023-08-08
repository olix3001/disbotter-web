<script lang="ts">
	import type { ENode } from "$lib/editor/node";
	import { getContext, onDestroy } from "svelte";
	import type { Writable } from "svelte/store";
	import NodeConnector from "./NodeConnector.svelte";

    export let node: ENode;
    export let currentZoom: number;

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
        }
    }
    function moveNode(e: MouseEvent) {
        if (e.buttons === 1 && dragging) {
            node.x += e.movementX * (1 / currentZoom);
            node.y += e.movementY * (1 / currentZoom);
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
            <NodeConnector color="white" style="double" />
        {/if}

        <div class="nvh-info">
            <p class="nvh-title">{node.type.title}</p>
            <p class="nvh-desc">{node.type.description}</p>
        </div>

        <!-- If node has __flow_out__ then it should be there -->
        {#if node.type.outputs.__flow_out__}
            <NodeConnector color="white" style="double" />
        {/if}
    </div>
    <div class="node-view-body">
        <div class="nvb-inputs">
            {#each Object.entries(node.type.inputs) as input}
                {#if input[0] !== "__flow_in__"}
                    <div class="nf-block nf-i">
                        <NodeConnector type={input[1].type} />
                        <p>{input[0]}</p>
                    </div>
                {/if}
            {/each}
        </div>
        <div class="nvb-outputs">
            {#each Object.entries(node.type.outputs) as output}
                {#if output[0] !== "__flow_out__"}
                    <div class="nf-block nf-o">
                        <p>{output[0]}</p>
                        <NodeConnector type={output[1].type} />
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
    }
    .nf-o {
        justify-content: flex-end;
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
