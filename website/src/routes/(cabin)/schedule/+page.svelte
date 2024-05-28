<script lang="ts">
	import Schedule from '$lib/client/Schedule.svelte';
	import DatePicker from '$lib/DatePicker.svelte';

	let selectingDate: boolean;
	let count = 0;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	on:click={() => {
		selectingDate = false;
	}}
>
	<!-- <DatePicker bind:selectingDate bind:count /> -->
	<Schedule />
</div>

<style>
	div {
		display: flex;
		width: 100%;
		height: 100%;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
