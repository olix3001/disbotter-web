
<script lang="ts">
	import { DisbotterProject } from "$lib/editor/project";
    import Navbar from "../../components/editor/Navbar.svelte";
	import ProjectProvider from "../../components/editor/ProjectProvider.svelte";
	import { Pane, Splitpanes } from "svelte-splitpanes";
	import ProjectStructure from "../../components/editor/ProjectStructure.svelte";
	import NodeEditor from "../../components/editor/nodes/NodeEditor.svelte";
	import Properties from "../../components/editor/Properties.svelte";
    import { writable } from "svelte/store";
	import BuildTime from "../../components/common/BuildTime.svelte";

    let project = writable(new DisbotterProject('New project'));

    async function handleMenuClick(e: any) {
        const action = e.detail.action;
        if (action == 'new') {
            $project = new DisbotterProject('New project');
        } else if (action == 'save') {
            $project.exportToFile();
        } else if (action == 'open') {
            await $project.ask_user_open();
            project.update(p => p);
        }
    }
</script>

<div style="width: 100vw; height: 100vh; overflow: hidden;">
    <Navbar on:menuclick={handleMenuClick}/>

    <div class="topbar">
        <p class="project-name">{$project.name}</p>
    </div>

    <ProjectProvider bind:PROJECT={project}>
        <Splitpanes style="height: 100%" theme="dark-splitpane-theme">
            <Pane minSize={12} size={15}>
                <div class="sidebar pane">
                    <ProjectStructure />
                </div>
            </Pane>
            <Pane minSize={40}>
                <div class="editor pane">
                    <NodeEditor />
                </div>
            </Pane>
            <Pane minSize={12} size={15}>
                <div class="properties pane">
                    <Properties />
                </div>
            </Pane>
        </Splitpanes>
    </ProjectProvider>
</div>

<BuildTime />

<style>
    .pane {
        color: var(--white);
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;
    }

    .sidebar, .properties {
        background-color: #0f0f0f;
    }

    .topbar {
        width: 100%;
        background-color: #0f0f0f;
        display: flex;
        flex-direction: row;
        border: 1.5px solid #313131;
    }

    .project-name {
        margin: 1em;
        font-weight: 700;
    }
</style>