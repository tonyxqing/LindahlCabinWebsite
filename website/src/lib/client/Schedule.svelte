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
			<div class="month_navigation_button_container">
				<button
					on:click={() => {
						focused = focused.prevMonth();
					}}>{`<`} prev</button
				>
				<button
					on:click={() => {
						focused = focused.nextMonth();
					}}>next {`>`}</button
				>
			</div>
			<CalendarComponent {ledger} {focused} bind:selectedDate {secondSelectedDate} />
		</div>
		<div class="visit_container">
			{#if selectedDate}
				Looking at: {selectedDate.toString()}
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
			{/if}
		</div>
	{/await}
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		flex: 1;
	}

	.visit_card {
		border: 1px solid gray;
		padding: 10px;
	}
	.calendar_container {
		display: flex;
		flex-direction: column;
		flex: 1 1 800px;
		width: 800px;
	}
	.month_navigation_button_container {
		display: flex;
		justify-content: space-between;
	}

	.visit_container {
		display: flex;
		flex: 1;
		flex-direction: column;
		width: 800px;
	}
</style>
