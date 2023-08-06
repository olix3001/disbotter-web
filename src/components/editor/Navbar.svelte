<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    const menu = [
        {
            name: 'File',
            items: [
                {
                    name: 'New',
                    action: 'new'
                },
                {
                    name: 'Open',
                    action: 'open'
                },
                {
                    name: 'Save',
                    action: 'save'
                },
                {
                    name: 'Save As',
                    action: 'saveAs'
                },
                {
                    name: 'Close',
                    action: 'close'
                }
            ]
        },
        {
            name: 'Project',
            items: [
                {
                    name: 'Settings',
                    action: 'settings'
                }
            ]
        }
    ];

    function handleMenuClick(event: string) {
        dispatch('menuclick', {
            action: event
        });
    }
</script>

<nav>
    <!-- Menu with dropdowns -->
    {#each menu as group}
        <div class="dropdown">
            <button class="dropbtn">{group.name}</button>
            <div class="dropdown-content">
                {#each group.items as item}
                    <button on:click={handleMenuClick.bind(null, item.action)}>{item.name}</button>
                {/each}
            </div>
        </div>
    {/each}
</nav>

<style>
    nav {
        background-color: #1f1f1f;
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: center;
        user-select: none;
        z-index: 9999;
    }

    button {
        background-color: #00000000;
        color: var(--white);
        border: none;
        padding: 0.3rem 0.7rem;
        font-size: 0.85rem;
        font-weight: 500;
        border-radius: 0.3rem;
        margin: 0.2rem;
        outline: none;
    }

    .dropbtn:hover, .dropbtn:has(+ .dropdown-content:hover) {
        background-color: #323232;
    }

    .dropdown {
        position: relative;
        display: inline-block;
    }

    .dropbtn:hover + .dropdown-content, .dropdown-content:hover {
        display: flex;
    }

    .dropdown-content {
        display: none;
        position: absolute;
        background-color: #181818;
        min-width: 160px;
        box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
        z-index: 10;
        border-radius: 5px;
        border: 1px solid #323232;
        transform: translateY(-5px);
        flex-direction: column;
        align-items: flex-start;
    }

    .dropdown-content button {
        width: calc(100% - 0.4rem);
        text-align: start;
        padding: 0.4rem 1.5rem;
        cursor: pointer;
    }
    .dropdown-content button:hover {
        background-color: #323232;
    }
</style>