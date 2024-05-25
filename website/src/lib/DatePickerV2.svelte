<script lang="ts">
	import { CalendarDate } from '$lib/client/calendarUtils';
	import CalendarComponent from './CalendarComponent.svelte';
	let date = new Date();

	let focused: CalendarDate = new CalendarDate(date);
	let focused2: CalendarDate = new CalendarDate(date).nextMonth();

	export let selectedDate: CalendarDate | undefined;
	export let secondSelectedDate: CalendarDate | undefined;
	export let mobile = false;
</script>

<section>
	<div class="navigation_container">
		<button
			on:click={() => {
				focused = focused.prevMonth();
				focused2 = focused2.prevMonth();
			}}>{`<`}</button
		>
		<button
			on:click={() => {
				focused = focused.nextMonth();
				focused2 = focused2.nextMonth();
			}}>{`>`}</button
		>
	</div>
	{#if mobile}
		<div class="mobile_calendar_container">
			<CalendarComponent bind:focused bind:selectedDate bind:secondSelectedDate />
			{#each Array(11).fill(0) as _, month}
				{@const focusedDate = new CalendarDate(focused.month, focused.day, focused.year).nextMonth(
					month + 1
				)}
				<CalendarComponent focused={focusedDate} bind:selectedDate bind:secondSelectedDate />
			{/each}
		</div>
	{:else}
		<div class="double_calendar_container">
			<CalendarComponent {focused} bind:selectedDate bind:secondSelectedDate />
			<CalendarComponent focused={focused2} bind:selectedDate bind:secondSelectedDate />
		</div>
	{/if}
</section>

<style>
	.double_calendar_container {
		display: flex;
		justify-content: center;
		gap: 16px;
	}
	.mobile_calendar_container {
		overflow: auto;
	}
	section {
		height: 500px;
		overflow: auto;
	}

	.navigation_container {
		display: flex;
		justify-content: space-between;
	}
</style>
