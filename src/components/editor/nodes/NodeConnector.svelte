<script lang="ts">
	import { NodeConnectionType, getNodeConnectionTypeColor, type ENode, type StructureType, compareStructureType } from "$lib/editor/node";
	import { type ProjectContext, projectKey } from "$lib/editor/project";
	import { getContext } from "svelte";

    export let isEndPort: boolean = false;
    export let node: ENode;
    export let key: string;
    export let color: string = "auto";
    export let type: NodeConnectionType = NodeConnectionType.Any;
    export let sType: StructureType = {};
    export let style: "default" | "double" = "default";
    export let port: SVGSVGElement;

    const PROJECT = getContext<ProjectContext>(projectKey);

    function startDrag(e: MouseEvent) {
        if (e.button == 0) {
            PROJECT.update((p) => {
                p.currentConnection = isEndPort ? {
                    type,
                    from: null,
                    fromKey: null,
                    to: node,
                    toKey: key,
                    sType
                } : {
                    type,
                    from: node,
                    fromKey: key,
                    to: null,
                    toKey: null,
                    sType
                };
                return p;
            });
        }
    }

    function stopDrag(e: MouseEvent) {
        if (e.button == 0) {
            const cc = $PROJECT.currentConnection;
            
            if (cc?.type !== type && cc?.type !== NodeConnectionType.Any) return;
            if (cc?.type == NodeConnectionType.Structure && !compareStructureType(cc?.sType ?? {}, sType)) return;

            if (cc?.from && cc?.fromKey) {
                PROJECT.update((p) => {
                    p.currentConnection = null;
                    p.createConnection({
                        type: cc.type,
                        from: cc.from,
                        fromKey: cc.fromKey,
                        to: node,
                        toKey: key,
                    });
                    return p;
                });
            } else if (cc?.to && cc?.toKey) {
                PROJECT.update((p) => {
                    p.currentConnection = null;
                    p.createConnection({
                        type: cc.type,
                        from: node,
                        fromKey: key,
                        to: cc.to,
                        toKey: cc.toKey,
                    });
                    return p;
                });
            }
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
    on:mouseup|stopPropagation={stopDrag}
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