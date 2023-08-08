<script lang="ts">
	import { NodeConnectionType, getNodeConnectionTypeColor, type ENode } from "$lib/editor/node";
	import { type ProjectContext, projectKey } from "$lib/editor/project";
	import { getContext } from "svelte";

    export let node: ENode;
    export let key: string;
    export let color: string = "auto";
    export let type: NodeConnectionType = NodeConnectionType.Any;
    export let style: "default" | "double" = "default";
    export let port: SVGSVGElement;

    const PROJECT = getContext<ProjectContext>(projectKey);

    function startDrag(e: MouseEvent) {
        if (e.button == 0) {
            PROJECT.update((p) => {
                p.currentConnection = {
                    type,
                    from: node,
                    fromKey: key,
                    to: null,
                    toKey: null,
                };
                console.log(p);
                return p;
            });
        }
    }

    $: nodeColor = color === "auto" ? getNodeConnectionTypeColor(type) : color;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<svg 
    class="nc" 
    class:nc-double={style==="double"} 
    bind:this={port}
    on:mousedown|stopPropagation={startDrag}
    >
    <!-- Circle -->
    <circle
        cx="5"
        cy="5"
        r={style==="double" ? "2" : "3"}
        fill={nodeColor}
        />
</svg>

<style>
    .nc {
        width: 10px;
        height: 10px;
        min-width: 10px;
        margin-right: 2px;
        margin-left: 2px;
        cursor: pointer;
    }

    .nc:hover {
        transform: scale(1.2);
    }

    .nc-double circle {
        outline-offset: 2px;
        outline: 1px solid white;
        border-radius: 50%;
    }
</style>