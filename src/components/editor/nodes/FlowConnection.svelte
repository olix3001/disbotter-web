<script lang="ts">
	import { getNodeConnectionTypeColor, type NodeConnection } from "$lib/editor/node";

    export let connection: NodeConnection;
    export let editor: HTMLDivElement;
    export let editorZoom: number;
    export let isCurrent: boolean = false;

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

    function calculateDynamicConnectionPath(e: MouseEvent) {
        if (isCurrent) {
            const parentR = editor.getBoundingClientRect();
            if (connection?.from && connection?.fromKey) {
                // First case, draw path from node to mouse
                const from = connection.from.oPorts[connection.fromKey]?.getBoundingClientRect();
                const fromX = (from.x + from.width/2 - parentR.x) / editorZoom
                const fromY = (from.y + from.height/2 - parentR.y) / editorZoom

                const toX = (e.clientX - parentR.x) / editorZoom
                const toY = (e.clientY - parentR.y) / editorZoom

                dynamicConnectionPath = `M ${fromX} ${fromY} C ${fromX + 75} ${fromY} ${toX - 75} ${toY} ${toX} ${toY}`;
            } else if (connection?.to && connection?.toKey) {
                // Second case, draw path from mouse to node
                const to = connection.to.iPorts[connection.toKey]?.getBoundingClientRect();
                const toX = (to.x + to.width/2 - parentR.x) / editorZoom
                const toY = (to.y + to.height/2 - parentR.y) / editorZoom

                const fromX = (e.clientX - parentR.x) / editorZoom
                const fromY = (e.clientY - parentR.y) / editorZoom

                dynamicConnectionPath = `M ${fromX} ${fromY} C ${fromX + 75} ${fromY} ${toX - 75} ${toY} ${toX} ${toY}`;
            }
        }
    }

    let dynamicConnectionPath = '';
    $: color = getNodeConnectionTypeColor(connection.type);
    $: pathCode = isCurrent ? dynamicConnectionPath : calculateConnectionPath(connection);
</script>

<path d={pathCode} fill="transparent" stroke={color} stroke-width="2"></path>
<svelte:body on:mousemove={calculateDynamicConnectionPath} />