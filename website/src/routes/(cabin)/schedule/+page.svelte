<script lang="ts">
	import Schedule from '$lib/client/Schedule.svelte';

	let selectingDate: boolean;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	on:click={() => {
		selectingDate = false;
	}}>
	<Schedule />
</div>

<style>
	div {
		display: flex;
		flex: 1;
	}
</style>
