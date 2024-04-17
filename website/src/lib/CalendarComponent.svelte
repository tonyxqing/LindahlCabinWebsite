<script lang="ts">
	import { onMount } from 'svelte';
	import { monthNames, daysOfWeek, CalendarDate, numDaysInMonth } from '$lib/client/calendarUtils';
	import FaUser from 'svelte-icons/fa/FaUser.svelte';
	import type { LedgerEntry } from '$lib';
	export let selectedDate: CalendarDate | undefined;
	export let secondSelectedDate: CalendarDate | undefined;
	export let focused: CalendarDate;
	export let ledger: { [key: string]: LedgerEntry[] };
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
		if (
			!ledger &&
			selectedDate &&
			!secondSelectedDate &&
			date.totalDays() > selectedDate.totalDays()
		) {
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
				class:right={!ledger && (active === 3 || (active === 2 && !secondSelectedDate))}
				class:left={!ledger && active === 2}
				class:invert={active === 1 && day === 0}
				class:invert-end={active === 1 && day + 1 === numDaysInMonth(focused.year)[focused.month]}
				class:month={true}
				class:inside={true}
			>
				<div class="day_of_month_tile">
					{#if day === 0}
						<p>{monthNames[focused.month].substring(0, 3)}</p>
					{/if}
					<p>
						{day + 1}
					</p>
				</div>
				{#if ledger}
					{@const day_string = `${focused.year}-${(focused.month + 1)
						.toString()
						.padStart(2, '0')}-${(day + 1).toString().padStart(2, '0')}`}
					{#if Array.isArray(ledger[day_string]) && ledger[day_string].length}
						<div style="width: 100%">
							{#each ledger[day_string] as entry, i}
								{#if ledger[day_string].length <= 2 || i === 0}
									<div class="ledger_visit">
										<img
											class="ledger_visit_image"
											src={entry.profile_pic}
											alt="avatar of visitor"
										/>
										<p class="ledger_visit_creator">
											{entry.name}
										</p>
									</div>
								{:else if i === 1}
									{@const str = ledger[day_string].reduce((agg, curr) => {
										return agg + curr.num_staying;
									}, 0) - 1}
									<div class="ledger_visit">
										<p class="ledger_visit_creator">
											+{str}
										</p>
										<div style="display: flex; height: 10px; color: red;">
											<FaUser />
										</div>
									</div>
								{/if}
							{/each}
						</div>
					{/if}
				{/if}
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
	.ledger_visit {
		display: flex;
		flex: 1;
		gap: 8px;
		align-items: center;
		border: 1px solid aliceblue;
		padding: 4px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	.ledger_visit_image {
		height: 14px;
		aspect-ratio: 1;
		border-radius: 50%;
	}

	.ledger_visit_creator {
		font-size: 9px;
		margin: 0;
		border-radius: 50%;
		color: red;
	}
	.day_of_month_tile {
		display: flex;
		gap: 4px;
		justify-content: start;
		width: 100%;
	}
	.day_of_month_tile > p {
		margin: 2px;
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
		padding: 4px;
		min-height: 20px;
		min-width: 20px;
		background-color: white;
		border: 1px solid black;
		flex-direction: column;
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
		background-color: white;
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
