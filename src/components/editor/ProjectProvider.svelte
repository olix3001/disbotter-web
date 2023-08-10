<script lang="ts">
	import { projectKey, DisbotterProject } from "$lib/editor/project";
    import { setContext } from "svelte";
	import type { Writable } from "svelte/store";

    export let PROJECT: Writable<DisbotterProject>;
    $: setContext(projectKey, PROJECT);
</script>

<slot />