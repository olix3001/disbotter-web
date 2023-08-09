<script lang="ts">
	import { getNodeConnectionTypeColor, type NodeConnection } from "$lib/editor/node";

    export let connection: NodeConnection;
    export let editor: HTMLDivElement;
    export let editorZoom: number;

    function calculateConnectionPath(conn: NodeConnection): string {
        if (conn?.from && conn?.fromKey && conn?.to && conn?.toKey) {
            const parentR = editor.getBoundingClientRect();
            const from = conn.from.oPorts[conn.fromKey]?.getBoundingClientRect();
            const to = conn.to.iPorts[conn.toKey]?.getBoundingClientRect();

            if (!from || !to) return '';

            const fromX = (from.x + from.width/2 - parentR.x) / editorZoom
            const fromY = (from.y + from.height/2 - parentR.y) / editorZoom

            const toX = (to.x + to.width/2 - parentR.x) / editorZoom
            const toY = (to.y + to.height/2 - parentR.y) / editorZoom

            return `M ${fromX} ${fromY} C ${fromX + 75} ${fromY} ${toX - 75} ${toY} ${toX} ${toY}`;
        }
        return '';
    }

    $: color = getNodeConnectionTypeColor(connection.type);
    $: pathCode = calculateConnectionPath(connection);
</script>

<path d={pathCode} fill="transparent" stroke={color} stroke-width="2"></path>