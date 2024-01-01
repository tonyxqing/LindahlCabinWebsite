<script lang="ts">
	import Calendar from '$lib/Calendar.svelte';
	import { daysOfWeek, monthNames, CalendarDate } from '$lib/client/calendarUtils';
	import DatePickerV2 from './DatePickerV2.svelte';
	let secondSelectedDate: CalendarDate | undefined;
	let selectedDate: CalendarDate | undefined;

	export let selectingDate = false;
	let guestInput: HTMLInputElement;
	let guestValue = 1;
	let w: number;
</script>

<section bind:clientWidth={w}>
	<div class="container destination">
		<sub>Destination</sub>
		<div>The Cabin</div>
	</div>
	<div
		role="none"
		on:click={(e) => {
			e.stopPropagation();
			selectingDate = true;
		}}
		class="container date"
	>
		<sub>Dates</sub>
		<div style="display: flex; gap: 12px;">
			{#if selectedDate}
				<div>
					{monthNames[selectedDate?.month].slice(0, 3)}
					{selectedDate?.day}
				</div>
			{:else}
				<div>----</div>
			{/if}
			-
			{#if secondSelectedDate}
				<div>
					{monthNames[secondSelectedDate?.month].slice(0, 3)}
					{secondSelectedDate?.day}
				</div>
			{:else}
				<div>----</div>
			{/if}
		</div>
	</div>
	<div
		role="none"
		on:click={() => {
			guestInput.focus();
		}}
		class="container"
	>
		<sub>Guests</sub>
		<div>
			<input
				bind:value={guestValue}
				bind:this={guestInput}
				style="cursor: pointer; background:transparent; color: white; border: none; outline: none"
				type="text"
			/>
		</div>
	</div>
	<div
		role="none"
		on:click={async () => {
			if (selectedDate && secondSelectedDate) {
				await addVisit(selectedDate?.toISOString(), secondSelectedDate?.toISOString(), guestValue);
			}
			selectedDate = undefined;
			secondSelectedDate = undefined;
		}}
		class="container schedule_trip"
	>
		Confirm Trip
	</div>
	<div
		role="none"
		on:click={(e) => {
			e.stopPropagation();
		}}
		class:calendar_container={true}
		class:hidden={!selectingDate}
	>
		<DatePickerV2 mobile={w < 400} bind:selectedDate bind:secondSelectedDate />
		<div
			role="none"
			class="select_dates_button"
			on:click={() => {
				selectingDate = false;
				guestInput.focus();
			}}
		>
			Done
		</div>
	</div>
</section>

<style>
	section {
		width: 100%;
		height: 100%;
	}
	@media (max-width: 720px) {
		section {
			flex-wrap: wrap;
			align-content: center;
			gap: 12px;
			margin: 24px;
		}
		.container {
			flex: 0 1 100%;
			padding: 12px;
			display: flex;
			border-radius: 12px;
			justify-content: space-around;
			flex-direction: column;
			border: 1px solid var(--border-color);
		}
	}
	.calendar_container {
		display: flex;
		flex-direction: column;
		align-items: center;
		position: absolute;
		background-color: var(--calendar-color);
		padding: 12px;
		border-radius: 12px;
		z-index: 100;
		height: 400px;
		overflow: auto;
	}
	.hidden {
		visibility: hidden;
	}
	section {
		display: flex;
		justify-content: center;
		align-items: center;
		overflow: hidden;
	}
	.destination {
		cursor: not-allowed !important;
		border-right: none;
		border-top-left-radius: 12px;
		border-bottom-left-radius: 12px;
	}
	.date {
		border-right: none;
		width: 160px !important;
	}
	.schedule_trip {
		border-top-right-radius: 12px;
		border-bottom-right-radius: 12px;
		align-items: center;
	}
	.container:hover {
		cursor: pointer;
		background-color: var(--calendar-color);
	}
	.container {
		width: 100px;
		height: 50px;
		padding: 12px;
		display: flex;
		justify-content: space-around;
		flex-direction: column;
		border: 1px solid var(--border-color);
	}
	.select_dates_button {
		display: flex;
		justify-content: center;
		align-items: center;
		background: var(--main-button);
		width: 100%;
		height: 30px;
		text-align: center;
		border-radius: 6px;
		cursor: pointer;
	}
	.select_dates_button:hover {
		background: blue;
	}
</style>
