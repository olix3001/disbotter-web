<script lang="ts">
	import { type ProjectContext, projectKey } from "$lib/editor/project";
	import { getContext } from "svelte";
	import NodeView from "./NodeView.svelte";
	import SearchContext from "./SearchContext.svelte";

    const PROJECT = getContext<ProjectContext>(projectKey);
    type Vec2 = { x: number, y: number };

    let editorOffset: Vec2 = { x: 0, y: 0 };
    $: transformCSS = `transform: translate(${editorOffset.x}px, ${editorOffset.y}px) scale(${editorZoom})`;
    let editorZoom: number = 1;
    $: PATTERN_SIZE = 5 * editorZoom;
    $: PATTERN_SIZE_HALF = PATTERN_SIZE * 0.1;
    let EDITOR: HTMLDivElement;
    let EDITOR_CONTENT: HTMLDivElement;

    let dragging = false;
    
    function zoomEditor(event: WheelEvent) {
        const zoomAmount = event.deltaY * -0.001;
        const zoom = editorZoom + zoomAmount;

        if (zoom > 0.1 && zoom < 5) {
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
        }
    }
    function endDragging(e: MouseEvent) {
        if (e.button == 0) {
            dragging = false;
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
    function closeContextMenu(e: MouseEvent) {
        if (e.button !== 2) {
            closeContext();
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
        on:mousedown={closeContextMenu}
        >
        <svg 
            class="editor-background"
            on:wheel|preventDefault={zoomEditor}
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
            <NodeView node={{
                id: 'onCommand',
                type: $PROJECT.commands[0].flow.availableNodes[0],

                x: 0,
                y: 0,

                inputs: {},
                outputs: {},
                inputHardcoded: {},
                outputHardcoded: {},
            }} currentZoom={editorZoom}/>
        </div>
        <SearchContext bind:openContext bind:closeContext />
     </div>
{/if}

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

    .editor-content {
        z-index: 999;
        position: absolute;
        top: 0;
        left: 0;
        transform-origin: top left;
    }
</style>