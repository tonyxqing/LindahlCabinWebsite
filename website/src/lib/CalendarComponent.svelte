<script lang="ts">
	import { onMount } from 'svelte';
	import { monthNames, daysOfWeek, CalendarDate, numDaysInMonth } from '$lib/client/calendarUtils';
	import FaUser from 'svelte-icons/fa/FaUser.svelte';
	import type { LedgerEntry } from '$lib';
	export let selectedDate: CalendarDate | undefined;
	export let secondSelectedDate: CalendarDate | undefined;
	export let focused: CalendarDate;
	export let ledger: {[key: string]: LedgerEntry[]};
	let inputDate: HTMLInputElement;
	let secondInputDate: HTMLInputElement;

	$: isActive = (day: number): number => {
		if (selectedDate) {
			const date = new Date(focused.year, focused.month, day + 1);
			const newFocused = new CalendarDate(date);
			if (newFocused.totalDays() === selectedDate?.totalDays()) {
				return 2;
			}
			if (secondSelectedDate) {
				if (newFocused.totalDays() === secondSelectedDate?.totalDays()) {
					return 3;
				}
				if (
					newFocused.totalDays() > selectedDate?.totalDays() &&
					newFocused.totalDays() < secondSelectedDate?.totalDays()
				)
					return 1;
			}
		}
		return 0;
	};
	$: handleClick = (day: number): void => {
		let date = new CalendarDate(focused.month, day, focused.year);
		if (!ledger && selectedDate && !secondSelectedDate && date.totalDays() > selectedDate.totalDays()) {
			[secondSelectedDate] = focused.handleClick(day);
		} else {
			[selectedDate] = focused.handleClick(day);
			secondSelectedDate = undefined;
		}
	};
	$: console.log(selectedDate, secondSelectedDate);

	onMount(() => {
		if (inputDate) {
			inputDate.value = focused.toString();
		}
	});
</script>

<div class="calendar_wrapper">
	<div class="month_navigation">
		<h1>
			{monthNames[focused.month] + ' ' + focused.year}
		</h1>
	</div>
	<header>
		<!-- mon tue wed thu fri  -->
		{#each daysOfWeek as dayName}
			<h3>{dayName.slice(0, 1)}</h3>
		{/each}
		<!-- blank days at start of month -->
		{#each Array(focused.beginningCalendarOffset()).fill(0) as _, day}
			<div class:month={true} class:outside={true} />
		{/each}
		<!-- numbered days in the month -->

		{#each Array(focused.numDaysInMonth()).fill(0) as _, day}
			{@const active = isActive(day)}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div
				role="cell"
				tabindex="0"
				on:click={() => handleClick(day)}
				class:active={active === 1 &&
					!(day === 0) &&
					!(day + 1 === numDaysInMonth(focused.year)[focused.month])}
				class:right={active === 3 || (active === 2 && !secondSelectedDate)}
				class:left={active === 2}
				class:invert={active === 1 && day === 0}
				class:invert-end={active === 1 && day + 1 === numDaysInMonth(focused.year)[focused.month]}
				class:month={true}
				class:inside={true}
			>
				{#if ledger}
					{@const day_string = `${focused.year}-${(focused.month + 1)
						.toString()
						.padStart(2, '0')}-${(day + 1).toString().padStart(2, '0')}`}

						{#if Array.isArray(ledger[day_string]) && ledger[day_string].length}
							<div style="position: absolute; top: 0; left: 0;">
								{#each ledger[day_string] as entry, i}
									<div style={`display: flex; gap: 8px; align-items:center;`}>
										<p
											style={`margin: 0; border-radius: 50%; aspectRatio: 1, width: 8px; color: red;`}
										>
											{entry.num_staying}
										</p>
										<div style="height: 11px; width: 11px; translate: 0px -4px;">
											<FaUser />
										</div>
									</div>
								{/each}
							</div>
						{/if}
				{/if}
				<p>
					{day + 1}
				</p>
			</div>
		{/each}
		<!-- blank days at the end of the month -->
		{#each Array(focused.endCalendarOffset()).fill(0) as _, day}
			<div class:month={true} class:outside={true} />
		{/each}
	</header>
</div>

<style>
	.outside {
		pointer-events: none;
	}
	.calendar_wrapper {
		display: flex;
		justify-content: center;
		flex-direction: column;
	}

	.month_navigation {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex: 1;
		gap: 8px;
		width: 100%;
		position: relative;
	}

	.month_navigation > h1 {
		height: 100%;
		display: flex;
		flex: 1;
		align-items: center;
		justify-content: center;
		padding: 0px;
		margin: 0px;
		gap: 8px;
		font-size: small;
	}
	.invert {
		background: linear-gradient(90deg, transparent, blue 50%) !important;
	}
	.invert > p {
		background-color: none !important;
		border: none !important;
	}

	.invert-end {
		background: linear-gradient(90deg, blue 50%, transparent) !important;
	}
	.invert-end > p {
		background-color: none !important;
		border: none !important;
	}
	header {
		padding-top: 16px;
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		text-align: center;
		background-color: transparent;
	}

	header > h3 {
		height: 100%;
		margin: 0px;
	}

	.month {
		cursor: pointer;
		aspect-ratio: 1;
		display: flex;
		flex: 1;
		justify-content: center;
		align-items: center;
		background-color: transparent;
		margin-top: 2px;
		margin-bottom: 2px;
		padding: 4px;
		min-height: 20px;
		min-width: 20px;
		position: relative;
		border: 1px solid black;
	}
	.month > p {
		font-size: 12px;
		font-weight: 600;
		margin: 0;
		display: flex;
		flex: 1;
		aspect-ratio: 1;
		border-radius: 50%;
		justify-content: center;
		align-items: center;
	}
	.month:hover > p {
		border: 1px solid rgb(255, 255, 255);
		border-radius: 50%;
	}
	.active > p {
		display: flex;
		flex: 1;
		aspect-ratio: 1;
		border-radius: 50%;
		justify-content: center;
		align-items: center;
	}
	.inside {
		background-color: transparent;
	}
	.active {
		background-color: blue;
	}

	.left {
		background-color: blue !important;
		border-top-left-radius: 50% !important;
		border-bottom-left-radius: 50% !important;
	}
	.right {
		background-color: blue !important;
		border-top-right-radius: 50% !important;
		border-bottom-right-radius: 50% !important;
	}
</style>
