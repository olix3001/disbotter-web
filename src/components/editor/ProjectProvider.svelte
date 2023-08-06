<script lang="ts">
	import { projectKey, type DisbotterProject } from "$lib/editor/project";
    import { setContext } from "svelte";

    export let project: DisbotterProject;

    setContext(projectKey, project);
</script>

<slot />