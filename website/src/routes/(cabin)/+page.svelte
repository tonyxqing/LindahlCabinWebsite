<script lang="ts">
	import DatePicker from '$lib/DatePicker.svelte';

	let selectingDate: boolean;
	let count = 0;
</script>

<section
	role="none"
	on:click={() => {
		selectingDate = false;
	}}
>
	<DatePicker bind:selectingDate bind:count />
</section>

<style>
	section {
		width: 100%;
		height: 100%;
		margin: auto;
		gap: 24px;
	}
</style>
