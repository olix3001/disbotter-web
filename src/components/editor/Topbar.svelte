<script lang="ts">
    import CompileLightIcon from "$lib/assets/compile_light.svg";
    import { PUBLIC_API_URL } from "$env/static/public";
	import { onMount } from "svelte";

    let api_available = false;
    
    let api_fetch: Promise<any>;
    
    onMount(() => {
        api_fetch = new Promise(async (resolve, reject) => {
            try {
                const response = await fetch(PUBLIC_API_URL + '/ping', {
                    method: 'GET'
                });

                if (response.status != 200) {
                    api_available = false;
                    reject(response);
                } else {
                    api_available = true;
                    resolve(response);
                }
            } catch (e) {
                api_available = false;
                reject(e);
            }
        });
    });
</script>

<div class="topbar">
    <div>
        <button disabled={!api_available}>
            <img src={CompileLightIcon} alt="Compile button"/>
            Compile
        </button>
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