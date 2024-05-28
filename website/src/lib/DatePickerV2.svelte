<script lang="ts">
	import { CalendarDate } from '$lib/client/calendarUtils';
	import CalendarComponent from './CalendarComponent.svelte';
	import FaChevronLeft from 'svelte-icons/fa/FaChevronLeft.svelte';
	import FaChevronRight from 'svelte-icons/fa/FaChevronRight.svelte';
	let date = new Date();

	let focused: CalendarDate = new CalendarDate(date);
	let focused2: CalendarDate = new CalendarDate(date).nextMonth();

	export let selectedDate: CalendarDate | undefined;
	export let secondSelectedDate: CalendarDate | undefined;
	export let mobile = false;
</script>

<section>
	{#if mobile}
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
		<div class="navigation_container">
			<button
			class="calendar_navigation_button"
			on:click={() => {
					focused = focused.prevMonth();
					focused2 = focused2.prevMonth();
			}}><FaChevronLeft /></button
		>
			<div class="double_calendar_container">
				<CalendarComponent {focused} bind:selectedDate bind:secondSelectedDate />
				<CalendarComponent focused={focused2} bind:selectedDate bind:secondSelectedDate />
			</div>
			<button
			class="calendar_navigation_button"
			on:click={() => {
					focused = focused.nextMonth();
					focused2 = focused2.nextMonth();
			}}><FaChevronRight/></button
		>

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
		overflow: auto;
	}

	.calendar_navigation_button {
		background-color: transparent;
		align-self: center;
		height: 48px;
		border: none;
		cursor: pointer;
		color: #c9c9c9;
	}

	.navigation_container {
		display: flex;
		justify-content: space-between;
	}
</style>
