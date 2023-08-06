<script lang="ts">
	import Collapse from "../common/Collapse.svelte";


</script>

<Collapse open={true}>
    <div slot="header">
        <p>Commands</p>
    </div>
    <div>
        <p>Commands go here...</p>
    </div>
</Collapse>
<Collapse open={true}>
    <div slot="header">
        <p>Events</p>
    </div>
    <div>
        <p>Events go here...</p>
    </div>
</Collapse>