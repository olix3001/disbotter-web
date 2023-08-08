<script lang="ts">
	import { NodeConnectionType, getNodeConnectionTypeColor } from "$lib/editor/node";

    export let color: string = "auto";
    export let type: NodeConnectionType = NodeConnectionType.Any;
    export let style: "default" | "double" = "default";

    $: nodeColor = color === "auto" ? getNodeConnectionTypeColor(type) : color;
</script>

<svg class="nc" class:nc-double={style==="double"}>
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
    }

    .nc-double circle {
        outline-offset: 2px;
        outline: 1px solid white;
        border-radius: 50%;
    }
</style>