<style global>
    :root {
        --white: #eeeeee;
    }

    :root {
        font-family: 'Roboto', sans-serif;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        background-color: #1f1f1f;
        color: var(--white);
    }

    :global(.splitpanes) {
        background-color: #1f1f1f;
    }

    :global(.splitpanes__splitter) {
        background-color: #313131;
        position: relative;
        width: 1.5px;
        cursor: ew-resize;
        transition: background-color 0.3s ease;
    }

    :global(.splitpanes__splitter:hover) {
        background-color: #057bd0;
    }
</style>

<slot />