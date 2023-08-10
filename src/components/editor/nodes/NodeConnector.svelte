<script lang="ts">
	import { NodeConnectionType, getNodeConnectionTypeColor, type ENode } from "$lib/editor/node";
	import { type ProjectContext, projectKey } from "$lib/editor/project";
	import { getContext } from "svelte";

    export let isEndPort: boolean = false;
    export let node: ENode;
    export let key: string;
    export let color: string = "auto";
    export let type: NodeConnectionType = NodeConnectionType.Any;
    export let sTags: string[] = [];
    export let port: SVGSVGElement;
    $: style = type === NodeConnectionType.Flow ? "double" : "default";

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
                    sTags
                } : {
                    type,
                    from: node,
                    fromKey: key,
                    to: null,
                    toKey: null,
                    sTags
                };
                return p;
            });
        }
    }

    function satisfiesTags(a: string[], b: string[]) {
        return b.every((tag) => a.includes(tag));
    }

    function stopDrag(e: MouseEvent) {
        if (e.button == 0) {
            const cc = $PROJECT.currentConnection;
            
            if (
                (cc?.type !== type && cc?.type !== NodeConnectionType.Any) ||
                (cc?.type == NodeConnectionType.Structure && (
                    (isEndPort && !satisfiesTags(cc.sTags??[], sTags)) ||
                    (!isEndPort && !satisfiesTags(sTags, cc.sTags??[]))
                ))
            ) {
                PROJECT.update((p) => {
                    p.currentConnection = null;
                    return p;
                });
                return;
            }

            if (cc?.from && cc?.fromKey && isEndPort) {
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
            } else if (cc?.to && cc?.toKey && !isEndPort) {
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
            } else {
                PROJECT.update((p) => {
                    p.currentConnection = null;
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