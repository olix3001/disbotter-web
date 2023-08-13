<script lang="ts">
    // @ts-ignore
    let build_time: string = import.meta.env.VITE_BUILD_TIME as string ?? "Unknown";
    // @ts-ignore
    let build_version: string = import.meta.env.VITE_COMMIT_HASH as string ?? "Unknown";
</script>


<div class="build-time-note">
    <!-- Build version -->
    <span class="build-time-note__text">Build version:</span>
    <span class="build-time-note__time">{build_version}</span>
    <br/>
    <!-- Build time -->
    <span class="build-time-note__text">Build time:</span>
    <span class="build-time-note__time">{new Date().toLocaleString()}</span>
</div>

<style>
    .build-time-note {
        position: fixed;
        bottom: 0;
        right: 0;
        background-color: #000;
        color: #fff;
        padding: 0.5rem;
        font-size: 0.75rem;
        font-family: monospace;
    }

    .build-time-note__text {
        font-weight: bold;
    }

    .build-time-note__time {
        font-weight: normal;
    }
</style>