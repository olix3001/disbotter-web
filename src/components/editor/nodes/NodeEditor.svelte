<script lang="ts">
	import { type ProjectContext, projectKey } from "$lib/editor/project";
	import { getContext, setContext } from "svelte";
	import NodeView from "./NodeView.svelte";
	import SearchContext from "./SearchContext.svelte";
	import type { NodeType } from "$lib/editor/node";
	import { writable } from "svelte/store";
	import FlowConnection from "./FlowConnection.svelte";
    import { v4 as uuidv4 } from "uuid";

    const PROJECT = getContext<ProjectContext>(projectKey);
    type Vec2 = { x: number, y: number };

    let editorOffset: Vec2 = { x: 0, y: 0 };
    $: transformCSS = `transform: translate(${editorOffset.x}px, ${editorOffset.y}px) scale(${editorZoom})`;
    let editorZoom: number = 1;
    $: PATTERN_SIZE = 10 * editorZoom;
    $: PATTERN_SIZE_HALF = PATTERN_SIZE * 0.1;
    let EDITOR: HTMLDivElement;
    let EDITOR_CONTENT: HTMLDivElement;

    let dragging = false;

    const selectedNodes = setContext("selectedNodes", writable([]))
    
    function zoomEditor(event: WheelEvent) {

        // If editor-context-search is part of the target, don't zoom
        if (event.target instanceof HTMLElement && event.target.closest(".editor-context-search")) return;

        event.preventDefault();

        const zoomAmount = event.deltaY * -0.001;
        const zoom = editorZoom + zoomAmount;

        if (zoom > 0.5 && zoom < 2) {
            const oldZoom = editorZoom;
            editorZoom = zoom;

            // Calculate offset to keep the same point under the cursor
            const rect = EDITOR.getBoundingClientRect();
            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;
            
            const offX = x - editorZoom / oldZoom * (x - editorOffset.x);
            const offY = y - editorZoom / oldZoom * (y - editorOffset.y);

            editorOffset.x = offX;
            editorOffset.y = offY;
        }
    }

    function moveEditor(event: MouseEvent) {
        if (event.buttons === 1 && dragging) {
            editorOffset.x += event.movementX;
            editorOffset.y += event.movementY;
        }
    }

    function startDragging(e: MouseEvent) {
        if (e.button == 0) {
            dragging = true;
            // Unselect all nodes
            selectedNodes.set([]);
        }
    }
    function endDragging(e: MouseEvent) {
        if (e.button == 0) {
            dragging = false;
            if ($PROJECT.currentConnection) {
                PROJECT.update(p => {
                    p.currentConnection = null;
                    return p;
                });
            }
        }
    }

    let openContext: ({ x, y }: { x: number, y: number }) => void;
    let closeContext: () => void;
    function openContextMenu(e: MouseEvent) {
        // Calcualte position relative to top left of editor
        const rect = EDITOR.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        openContext({ x, y });
    }

    function insertNode(e: { detail: {node: NodeType, position: { x: number, y: number }} }) {
        const { node, position } = e.detail;

        // Calculate position to insert node at
        const x = (position.x - editorOffset.x) / editorZoom;
        const y = (position.y - editorOffset.y) / editorZoom;

        PROJECT.update(project => {
            project.addNode(
                {
                    uid: uuidv4(),
                    type: node,
                    x: x,
                    y: y,
                    iPorts: {},
                    oPorts: {},
                    inputHardcoded: window.structuredClone(node.defaultHardcoded ?? {}),
                }
            )
            return project;
        });
    }

    function keypress(e: KeyboardEvent) {
        if (e.key === "Delete") {
            PROJECT.update(project => {
                project.removeNodes($selectedNodes);
                return project;
            });
        }
    }
</script>

{#if $PROJECT.currentlyEditing === null}
    <div class="not-editing">
        <p class="ne-title">Currently not editing.</p>
        <p class="ne-desc">Select something to edit from the panel on the left.</p>
    </div>
{:else}
     <!-- svelte-ignore a11y-no-static-element-interactions -->
     <div 
        class="editor"
        bind:this={EDITOR}
        on:contextmenu|preventDefault|stopPropagation={openContextMenu}
        on:wheel={zoomEditor}
        >
        <svg 
            class="editor-background"
            on:mousedown={startDragging}
            on:mouseup={endDragging}
            on:mousemove|preventDefault={moveEditor}
        >
            <defs>
                <pattern id="dots" width={PATTERN_SIZE} height={PATTERN_SIZE} patternUnits="userSpaceOnUse" x={editorOffset.x} y={editorOffset.y}>
                    <circle cx={PATTERN_SIZE_HALF} cy={PATTERN_SIZE_HALF} r={PATTERN_SIZE_HALF} fill="var(--editor-dots)" />
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#dots)"></rect>
        </svg>

        <div class="editor-content" bind:this={EDITOR_CONTENT} style={transformCSS}>
            <svg class="editor-connections">
                {#if $PROJECT.currentConnection !== null}
                    <!-- Current Connection -->
                    <FlowConnection connection={$PROJECT.currentConnection} editor={EDITOR_CONTENT} editorZoom={editorZoom} isCurrent={true}/>
                {/if}
                {#each $PROJECT.getCurrentFlow()?.connections ?? [] as conn}
                    <FlowConnection connection={conn} editor={EDITOR_CONTENT} editorZoom={editorZoom}/>
                {/each}
            </svg>
            {#each $PROJECT.getCurrentFlow()?.nodes ?? [] as node (node.uid)}
                <NodeView node={node} currentZoom={editorZoom} />
            {/each}
        </div>
        <SearchContext bind:openContext bind:closeContext on:nodeselected={insertNode} />
     </div>
{/if}

<svelte:document on:keydown={keypress} />

<style>
    .not-editing {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 50%;
    }

    .not-editing .ne-title {
        color: var(--white);
        font-weight: 700;
        font-size: xx-large;
    }
    .not-editing .ne-desc {
        color: var(--white);
        font-weight: 500;
        font-size: large;
    }

    .editor {
        height: 100%;
        width: 100%;
        position: relative;
        overflow: clip;
        background-color: var(--editor-background);
    }

    .editor-background {
        height: 100%;
        width: 100%;
        cursor: move;
        z-index: 1;
        position: absolute;
        top: 0;
        left: 0;
    }

    .editor-connections {
        z-index: 1000;
        position: absolute;
        top: 0;
        left: 0;
        overflow: visible;
        pointer-events: none;
    }

    .editor-content {
        z-index: 999;
        position: absolute;
        top: 0;
        left: 0;
        transform-origin: top left;
    }
</style>