<script lang="ts">
	import { getVisits, removeVisit, type Visit } from '$lib/client/serverComms';
	import { onMount } from 'svelte';
	let visits: Visit[] = [];
	onMount(async () => {
		visits = await getVisits();
	});
	function toUTC(date: string): number {
		const [year, month, day]: number[] = date
			.split('T')[0]
			.split('-')
			.map((x, i) => {
				if (i === 1) {
					return parseInt(x) - 1;
				}
				return parseInt(x);
			});
		return Date.UTC(year, month, day);
	}
</script>

<div class="container">
	{#each visits as { id, creatorId, arrival, departure, numStaying, postedOn }}
		{@const a = new Date(toUTC(arrival))}
		{@const b = new Date(toUTC(departure))}
		{@const c = new Date(Date.parse(postedOn.split(' ')[0]))}
		<div class="visit_container">
			<button
				class="remove_visit_button"
				on:click={async () => {
					await removeVisit(id);
					visits = await getVisits();
				}}>x</button
			>
			<div>arrival: {a.toISOString()} - departure: {b.toISOString()}</div>
			<div>{numStaying}</div>
			<div>{c.toISOString()}</div>
		</div>
	{/each}
</div>

<style>
	.remove_visit_button {
		border-color: red;
		background-color: transparent;
		float: right;
		color: white;
		border-radius: 50%;
	}
	.visit_container {
		padding: 8px;
		border-radius: 8px;
		border-width: 1px;
		border-style: solid;
		border-color: blueviolet;
		width: 480px;
		margin: 3px auto;
	}
	.container {
	}
</style>
