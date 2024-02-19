<script lang="ts">
	import DatePickerV2 from '$lib/DatePickerV2.svelte';
	import DatePicker from '$lib/DatePicker.svelte';

	import CalendarComponent from '$lib/CalendarComponent.svelte';
	import { CalendarDate } from '$lib/client/calendarUtils';
	import VisitContainer from '$lib/client/VisitContainer.svelte';
	let selectingDate: boolean;
	// The current page on the calendar
	let count = 0;
</script>

<section
	role="none"
	on:click={() => {
		selectingDate = false;
	}}
>
	<!-- <DatePicker bind:selectingDate bind:selectedDate bind:secondSelectedDate /> -->
	<!-- <DatePickerV2 bind:selectingDate bind:focused bind:selectedDate bind:secondSelectedDate/> -->
	<DatePicker bind:selectingDate bind:count />
	{#key count}
	    <VisitContainer/>
	{/key}
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
	}
	:global(button) {
		background-color: #f9f9f9;
	}

	section {
		width: 100%;
		height: 100%;
		gap: 24px;
		
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
			height: 100%;
			min-height: 100vh;
		}
		:global(body) {
			height: 100%;
			min-height: 100vh;
		}
	}
</style>
