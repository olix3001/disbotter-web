<script lang="ts">
    import CompileLightIcon from "$lib/assets/compile_light.svg";
    import BugLightIcon from "$lib/assets/bug_light.svg";
    import { PUBLIC_API_URL } from "$env/static/public";
	import { getContext, onMount } from "svelte";
	import { type ProjectContext, projectKey } from "$lib/editor/project";

    let api_available = false;
    let api_can_run = false;
    
    let api_fetch: Promise<any>;
    
    onMount(() => {
        api_fetch = new Promise(async (resolve, reject) => {
            try {
                const response = await fetch(PUBLIC_API_URL + '/config', {
                    method: 'GET'
                });

                if (response.status != 200) {
                    api_available = false;
                    api_can_run = false;
                    reject(response);
                } else {
                    api_available = true;
                    
                    const data = await response.json();
                    api_can_run = data.can_run;

                    resolve(response);
                }
            } catch (e) {
                api_available = false;
                api_can_run = false;
                reject(e);
            }
        });
    });

    const PROJECT = getContext<ProjectContext>(projectKey);

    function handleCompileClick() {
        $PROJECT.compileWithApi();
    }

    async function handleRunClick() {
        const resp = $PROJECT.testWithApi();
        console.log(await resp);
    }
</script>

<div class="topbar">
    <div>
        <button disabled={!api_available} on:click={handleCompileClick}>
            <img src={CompileLightIcon} alt="Compile button"/>
            Compile
        </button>
        <button disabled={!api_available || !api_can_run} on:click={handleRunClick}
            title={!api_can_run ? "Running is disabled on current API":""}>
            <img src={BugLightIcon} alt="Test button"/>
            Test
    </div>
    
    {#await api_fetch}
        <p class="api-waiting-message">Waiting for API.</p>
    {:then _value}
        <p class="api-success-message">API is available.</p>
    {:catch _error}
        <p class="api-error-message">Could not connect to API.</p>
    {/await}
</div>

<style>
    .topbar {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        padding: 0 1rem;
        height: 2rem;
        width: 100%;
    }

    .topbar .api-error-message {
        color: #ee5339;
        font-size: 0.8rem;
        font-weight: 700;
    }

    .topbar .api-waiting-message {
        color: #eeab2c;
        font-size: 0.8rem;
        font-weight: 700;
    }

    .topbar .api-success-message {
        color: #2f904b;
        font-size: 0.8rem;
        font-weight: 700;
    }

    .topbar div {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .topbar button {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        background-color: transparent;
        border: none;
        color: var(--white);
        font-weight: 700;
        font-size: 0.8rem;
        cursor: pointer;
        border-radius: 5px;
        padding: .5em;
    }

    .topbar button:disabled {
        cursor: not-allowed;
        color: #555;
    }

    .topbar button:hover {
        background-color: #1f1f1f;
    }

    .topbar button img {
        width: 1.3rem;
        height: 1.3rem;
        margin-right: 0.5rem;
    }

</style>