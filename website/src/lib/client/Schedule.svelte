<script lang="ts">
	import CalendarComponent from '$lib/CalendarComponent.svelte';
	import { CalendarDate } from '$lib/client/calendarUtils';
	import { getVisits } from '$lib/client/serverComms';
	let focused = new CalendarDate(new Date());
	let selectedDate: CalendarDate;
	let secondSelectedDate: CalendarDate;
	let visits = getVisits();
</script>

<div>
	{#await visits then visits}
		<CalendarComponent noSelect {focused} bind:selectedDate {secondSelectedDate} {visits} />
	{/await}
</div>
