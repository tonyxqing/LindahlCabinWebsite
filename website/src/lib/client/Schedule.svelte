<script lang="ts">
	import CalendarComponent from '$lib/CalendarComponent.svelte';
	import { CalendarDate } from '$lib/client/calendarUtils';
	import { getLedger, getMembers } from '$lib/client/serverComms';
	import { onMount } from 'svelte';
	let focused = new CalendarDate(new Date());
	let selectedDate: CalendarDate;
	let secondSelectedDate: CalendarDate;
	let usersDict;
	$: ledger = getLedger(focused.getDate().toISOString());
	$: (async () => console.log(await users))();
	onMount(async () => {
		let users = await getMembers();
		usersDict = users.reduce((agg, curr) => {
			let { id, ...rest } = curr;
			agg[id] = rest;
			return agg;
		}, {});
	});
</script>

<main>
	{#await ledger}
		<p>Loading</p>
	{:then ledger}
		<div class="calendar_container">
			<button
				on:click={() => {
					focused = focused.nextMonth();
				}}>next</button
			>
			<button
				on:click={() => {
					focused = focused.prevMonth();
				}}>prev</button
			>
			<CalendarComponent {ledger} {focused} bind:selectedDate {secondSelectedDate} />
		</div>
		<div class="visit_container">
			{#if selectedDate && usersDict}
				Looking at: {selectedDate.toString()}
				{#if ledger[selectedDate.toString()] && ledger[selectedDate.toString()].length}
					{#each ledger[selectedDate.toString()] as visit}
						{@const user = usersDict[visit.creator_id]}
						{#if user}
							<div class="visit_card">
								<p>
									{JSON.stringify(visit)}
									{JSON.stringify(user)}
								</p>
								{#if user.profilePic}
									<img src={user.profilePic} alt="profile" />
								{/if}
							</div>
						{/if}
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
		flex: 1;
	}

	.visit_card {
		border: 1px solid gray;
		padding: 10px;
	}
	.calendar_container {
		display: flex;
		flex-direction: column;
		flex: 1;
		flex-basis: 100%;
	}

	.visit_container {
		display: flex;
		flex: 1;
		flex-direction: column;
		width: 200px;
	}
</style>
