<script lang="ts">
	import DatePickerV2 from '$lib/DatePickerV2.svelte';
	import DatePicker from '$lib/DatePicker.svelte';

	import CalendarComponent from '$lib/CalendarComponent.svelte';
	import { CalendarDate } from '$lib/client/calendarUtils';
	let selectingDate: boolean;
	// The current page on the calendar
	let date = new Date();
	let focused = new CalendarDate(date);
	let selectedDate: CalendarDate;
	let secondSelectedDate: CalendarDate;
</script>

<section
	role="none"
	on:click={() => {
		selectingDate = false;
	}}
>
	<DatePicker bind:selectingDate bind:selectedDate bind:secondSelectedDate />
	<!-- <DatePickerV2 bind:selectingDate bind:focused bind:selectedDate bind:secondSelectedDate/> -->
</section>

<style>
	:global(:root) {
		--main-button: rgb(168, 0, 0);
		--confirm-button: rgba(29, 208, 29, 0.52);
		--calendar-color: #cfbcbc;
		--active-color: #626275;
		--border-color: #000;
		display: flex;
		justify-content: center;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
			Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
		background-color: #ffffff;
		width: 100%;
		height: 100%;
	}
	:global(button) {
		background-color: #f9f9f9;
	}
	:global(body) {
		width: 100%;
		height: 100%;
	}
	section {
		display: flex;
		width: 100%;
		height: 100%;
		gap: 24px;
		justify-content: center;
	}
	@media (prefers-color-scheme: dark) {
		:global(:root) {
			--main-button: rgb(94, 124, 220);
			--confirm-button: rgb(230, 215, 0);
			--calendar-color: #2a2a2a;
			--border-color: #fff;
			--active-color: #050508;
			color: #dfe1e2;
			background-color: #02041c;
		}
		:global(button) {
			--main-button: rgb(94, 124, 220);
		}
		:global(.month) {
			background-color: rgb(47, 47, 47);
		}
	}
</style>
