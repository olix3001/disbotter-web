<script lang="ts">
	import { projectKey, DisbotterProject } from "$lib/editor/project";
    import { setContext } from "svelte";
	import { writable } from "svelte/store";

    export let project: DisbotterProject;
    $: PROJECT = writable<DisbotterProject>(project || new DisbotterProject('Not Loaded')); 
    $: setContext(projectKey, PROJECT);
</script>

<slot />