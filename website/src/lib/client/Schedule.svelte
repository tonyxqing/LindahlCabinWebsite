<script lang="ts">
	import CalendarComponent from '$lib/CalendarComponent.svelte';
	import { CalendarDate } from '$lib/client/calendarUtils';
	import { getLedger, getMembers, type UserDict, type UserDTO } from '$lib/client/serverComms';
	import { onMount } from 'svelte';
	let focused = new CalendarDate(new Date());
	let selectedDate: CalendarDate;
	let secondSelectedDate: CalendarDate;
	$: ledger = getLedger(focused.getDate().toISOString());
</script>

<main>
	{#await ledger}
		<p>Loading</p>
	{:then ledger}
		<div class="calendar_container">
			<CalendarComponent {ledger} {focused} bind:selectedDate {secondSelectedDate} />
		</div>
		<div class="visit_container">
			<h3>
				{#if selectedDate}
					Looking at: {selectedDate?.toString()}
				{:else}
					Select a date to view details.
				{/if}
			</h3>
			{#if selectedDate}
				<div class="visits">
					{#if ledger[selectedDate.toString()] && ledger[selectedDate.toString()].length}
						{#each ledger[selectedDate.toString()] as visit}
							<div class="visit_card">
								{#if visit.profile_pic}
									<img src={visit.profile_pic} alt="profile" />
								{/if}
								{#if visit}
									<p>Arrival: {visit.arrival}</p>
									<p>Departure: {visit.departure}</p>
									<p>Posted On: {visit.posted_on}</p>
									<p>People Staying: {visit.num_staying}</p>
								{/if}
							</div>
						{/each}
					{:else}
						<p>No visits</p>
					{/if}
				</div>
			{/if}
		</div>
	{:catch err}
		<div>
			Unable to load calendar: Error ({err})
		</div>
	{/await}
</main>

<style>
	main {
		display: flex;
		justify-content: space-evenly;
		width: 100%;
		overflow: auto;
		padding: 24px;
	}

	.visit_card {
		display: flex;
		border: 1px solid gray;
		padding: 10px;
	}
	.calendar_container {
		display: flex;
		flex-direction: column;
		flex: 0 1 600px;
		width: 600px;
	}
	.visits {
		height: 100%;
		overflow-y: auto;
		overflow-x: hidden;
	}
	.visit_container {
		display: flex;
		flex-direction: column;
		height: 80vh;
		width: 300px;
		padding: 12px;
		color: white;
		overflow: hidden;
		position: sticky;
		top: 0;
	}
</style>
