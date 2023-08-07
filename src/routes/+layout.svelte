<style global>
    :root {
        --white: #eeeeee;
        --editor-background: #3f3f3f;
    }

    :root {
        font-family: 'Roboto', sans-serif;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        background-color: #1f1f1f;
        color: var(--white);
        user-select: none;
    }

    :global(.splitpanes) {
        background-color: #1f1f1f;
    }

    :global(.splitpanes__pane) {
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

    :global(p) {
        margin: 0;
    }
</style>

<slot />