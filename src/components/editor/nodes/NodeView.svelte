<script lang="ts">
	import type { ENode } from "$lib/editor/node";
	import { getContext, onDestroy } from "svelte";
	import type { Writable } from "svelte/store";

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
            <!-- TODO: Put flow input connection here -->
        {/if}

        <div>
            <p class="nvh-title">{node.type.title}</p>
            <p class="nvh-desc">{node.type.description}</p>
        </div>

        <!-- If node has __flow_out__ then it should be there -->
        {#if node.type.outputs.__flow_out__}
            <!-- TODO: Put flow output connection here -->
        {/if}
    </div>
    <div class="node-view-body">
        <div class="node-view-body-content">
            
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
        min-width: 130px;
    }

    .node-view-header {
        background-color: #0f0f0f;
        display: flex;
        flex-direction: row;
        align-items: center;
        padding: .35em;
        cursor: pointer;
    }

    .node-view-body {
        display: flex;
        flex-direction: column;
        background-color: #1f1f1f;
        border: 2px solid #1f1f1f;
        padding: 0.5em;
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
