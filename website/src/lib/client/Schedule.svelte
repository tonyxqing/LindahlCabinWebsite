<script lang="ts">
	import CalendarComponent from '$lib/CalendarComponent.svelte';
	import { CalendarDate } from '$lib/client/calendarUtils';
	import { addVisit, getLedger, getVisits } from '$lib/client/serverComms';
	import FaPlusCircle from 'svelte-icons/fa/FaPlusCircle.svelte';
	import DatePickerV2 from '$lib/DatePickerV2.svelte';
	let focused = new CalendarDate(new Date());
	let selectedDate: CalendarDate | undefined;
	let secondSelectedDate: CalendarDate | undefined;
	$: fetched_ledger = getLedger(focused.getDate().toISOString());
	let selectingDate = false;
	let guestInput: HTMLInputElement;
	let guestValue = 1;
	let visits = getVisits();
	let count = 0;
	let w: number;
</script>

<main bind:clientWidth={w}>
	{#await fetched_ledger}
		<p>Loading</p>
	{:then ledger}
		<div class="calendar_header_container">
			<div class="visit_container">
				<h3>UPCOMING TRIPS</h3>
				{#await visits then visits}
					{#if visits.length}
						{#each visits as visit}
							<div class="visit_card">
								<span>
									<img
										src={visit.profilePic}
										class="visit_profile_pic"
										height="30px"
										width="30px"
										alt="trip posted by" />
									<p>{visit.name}</p>
								</span>
								<p>{visit.numStaying}</p>
								<span>
									<p>{visit.arrival}</p>
									<p>{visit.departure}</p>
								</span>
							</div>
						{/each}
					{:else}
						<p>No upcoming trips.</p>
					{/if}
				{/await}

				<!-- <h3>
					{#if selectedDate}
						Upcoming Visits: {selectedDate?.toString()}
					{:else}
						Select a date to view details.
					{/if}
				</h3>
				{#if selectedDate}
					<div class="visits">
						{#if ledger[selectedDate.toString()] && ledger[selectedDate.toString()].length}
							{#each ledger[selectedDate.toString()] as visit}
								<div class="visit_card">
									<div class="visit_card_header">
										<div class="visit_profile">
											{#if visit.profile_pic}
												<img src={visit.profile_pic} alt="profile" />
											{:else}
												<FaUser />
											{/if}
											<p>{visit.name}</p>
										</div>
										{#if visit.creator_id === $auth.sub}
											<button
												class="delete_button"
												on:click={async () => {
													await removeVisit(visit.id);
													fetched_ledger = getLedger(focused.getDate().toISOString());
												}}><FaTrash /></button
											>
										{/if}
									</div>
									{#if visit}
										<div class="visit_card_content">
											<div class="visit_card_header">
												<div>
													<h6>Arrival</h6>
													<p>
														{new Date(visit.arrival).toLocaleString('en-us', {
															year: 'numeric',
															month: '2-digit',
															day: '2-digit'
														})}
													</p>
												</div>
												<div>
													<h6>Departure</h6>
													<p>
														{new Date(visit.departure).toLocaleString('en-us', {
															year: 'numeric',
															month: '2-digit',
															day: '2-digit'
														})}
													</p>
												</div>
											</div>
											<div>
												<h6>People Staying</h6>
												<p>{visit.num_staying}</p>
											</div>
											<small>{visit.posted_on}</small>
										</div>
									{/if}
								</div>
							{/each}
						{:else}
							<p>No visits</p>
						{/if}
					</div>
				{/if} -->
			</div>
			<button
				class="add_visit_button"
				on:click={(e) => {
					e.stopPropagation();
					selectingDate = true;
				}}>
				<div class="add_visit_icon">
					<FaPlusCircle />
				</div>
				<p>ADD NEW TRIP TO CALENDAR</p>
			</button>
			<div
				role="none"
				on:click={(e) => {
					e.stopPropagation();
				}}
				class:select_calendar_container={true}
				class:hidden={!selectingDate}>
				<DatePickerV2 mobile={w < 400} bind:selectedDate bind:secondSelectedDate />
				<div>
					People Coming <input type="number" max="20" />
				</div>
				<div class="calendar_button_container">
					<button
						class="select dates_button"
						on:click={async () => {
							if (selectedDate && secondSelectedDate) {
								console.log(
									selectedDate?.getDate().toISOString(),
									secondSelectedDate?.getDate().toISOString()
								);
								await addVisit(
									selectedDate?.getDate().toISOString(),
									secondSelectedDate?.getDate().toISOString(),
									guestValue
								);
								count++;
							}
							selectedDate = undefined;
							secondSelectedDate = undefined;
							selectingDate = false;
							visits = getVisits();
							fetched_ledger = getLedger(focused.getDate().toISOString());
						}}>
						Done
					</button>
					<button
						class="cancel dates_button"
						on:click={() => {
							selectingDate = false;
							guestInput.focus();
						}}>
						Cancel
					</button>
				</div>
			</div>
		</div>
		<div class="calendar_container">
			<CalendarComponent {ledger} {focused} bind:selectedDate {secondSelectedDate} />
		</div>
	{:catch err}
		<div>
			Unable to load calendar: Error ({err})
		</div>
	{/await}
</main>

<style>
	.calendar_header_container {
		flex: 1;
		padding: 12px;
		justify-content: center;
		align-items: center;
		display: flex;
		flex-direction: column;
	}
	main {
		display: flex;
		justify-content: space-evenly;
		width: 100%;
		padding: 24px;
		flex-wrap: wrap;
	}

	.calendar_button_container {
		display: flex;
		gap: 20px;
	}

	.add_visit_icon {
		width: 80px;
	}
	.hidden {
		visibility: hidden;
	}
	.select_calendar_container {
		display: flex;
		flex-direction: column;
		align-items: center;
		position: absolute;
		background-color: white;
		box-shadow: 0px 0px 3px 3px gray;
		padding: 12px;
		border-radius: 12px;
		z-index: 100;
		overflow: auto;
	}

	.dates_button {
		font-weight: 600;
		border-radius: 8px;
		padding: 4px;
	}

	.dates_button:hover {
		cursor: pointer;
		opacity: 0.6;
	}

	.select {
		border: 1px solid var(--border-color);
		color: var(--text-color);
	}

	.cancel {
		border: 1px solid red;
		color: red;
	}

	p,
	h3 {
		margin: 0;
		color: black;
	}
	.calendar_container {
		display: flex;
		align-items: center;
	}

	.add_visit_button {
		display: flex;
		align-items: center;
		background-color: transparent;
		justify-content: space-around;
		border: 1px solid transparent;
		height: 100px;
		border-radius: 8px;
		color: var(--text-color);
		gap: 4px;
		width: 90%;
		padding: 12px;
	}
	.visit_profile_pic {
		border-radius: 50%;
	}

	.add_visit_button:hover {
		cursor: pointer;
		border: 1px solid var(--border-color);
	}
	.add_visit_button > p {
		color: var(--text-color);
		font-weight: 600;
		font-size: 20px;
	}
	.visit_container {
		display: flex;
		flex: 1;
		width: 90%;
		margin: 8px;
		border-radius: 12px;
		flex-direction: column;
		padding: 12px;
		align-items: center;
		background-color: #dfe5f1;
		color: white;
		overflow: hidden;
		top: 0;
	}
</style>
