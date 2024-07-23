<script lang="ts">
	import { onMount } from 'svelte';
	import { monthNames, daysOfWeek, CalendarDate, numDaysInMonth } from '$lib/client/calendarUtils';
	import FaUser from 'svelte-icons/fa/FaUser.svelte';
	import FaChevronLeft from 'svelte-icons/fa/FaChevronLeft.svelte';
	import FaChevronRight from 'svelte-icons/fa/FaChevronRight.svelte';
	import type { LedgerEntry } from '$lib';
	export let selectedDate: CalendarDate | undefined;
	export let secondSelectedDate: CalendarDate | undefined;
	export let focused: CalendarDate;
	export let ledger: { [key: string]: LedgerEntry[] } | undefined = undefined;
	let inputDate: HTMLInputElement;

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
			date.totalDays() >= selectedDate.totalDays()
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

{#if ledger}
	<button
		class="calendar_navigation_button"
		on:click={() => {
			focused = focused.prevMonth();
		}}><FaChevronLeft /></button
	>
{/if}
<div class="calendar_wrapper">
	<div class="month_navigation">
		<h1 class:small_calendar_header={!ledger}>
			{monthNames[focused.month].toUpperCase()}{ledger ? '' : ' ' + focused.year}
		</h1>
	</div>
	<header>
		<!-- mon tue wed thu fri  -->
		{#each daysOfWeek as dayName}
			<h3>{ledger ? dayName.toUpperCase().slice(0,3) : dayName.slice(0, 1)}</h3>
		{/each}
		<!-- blank days at start of month -->
		{#each Array(focused.beginningCalendarOffset()).fill(0) as _, day}
			<div class:month={true} class:large_tile={ledger} class:outside={true} />
		{/each}
		<!-- numbered days in the month -->
		{#each Array(focused.numDaysInMonth()).fill(0) as _, day}
			{@const active = isActive(day)}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div
				role="cell"
				tabindex="0"
				on:click={() => handleClick(day)}
				class:active={!ledger &&
					active === 1 &&
					!(day === 0) &&
					!(day + 1 === numDaysInMonth(focused.year)[focused.month])}
				class:right={!ledger && (active === 3 || (active === 2 && !secondSelectedDate))}
				class:left={!ledger && active === 2}
				class:invert={!ledger && active === 1 && day === 0}
				class:large_tile={ledger}
				class:invert-end={!ledger &&
					active === 1 &&
					day + 1 === numDaysInMonth(focused.year)[focused.month]}
				class:month={true}
				class:inside={true}
				class:center={!ledger}
			>
				<div class="day_of_month_tile" class:center={!ledger}>
					{#if day === 0 && ledger}
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
									{@const str =
										ledger[day_string].reduce((agg, curr) => {
											return agg + curr.num_staying;
										}, 0) - 1}
									<div class="ledger_visit">
										<p class="ledger_visit_creator">
											+{str}
										</p>
										<div class="ledger_user_icon">
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
			<div class:month={true} class:large_tile={ledger} class:outside={true} />
		{/each}
	</header>
</div>
{#if ledger}
	<button
		class="calendar_navigation_button"
		on:click={() => {
			focused = focused.nextMonth();
		}}><FaChevronRight /></button
	>
{/if}

<style>
	.calendar_navigation_button {
		background-color: transparent;
		height: 48px;
		border: none;
		cursor: pointer;
		color: #c9c9c9;
	}
	.outside {
		pointer-events: none;
	}
	.calendar_wrapper {
		display: flex;
		justify-content: center;
		flex-direction: column;
		border-right: 1px solid var(--border-color);
		box-shadow: 4px 4px 5px lightslategray;
	}
	.ledger_user_icon {
		display: flex;
		height: 10px;
		color: var(--text-color);
	}
	.ledger_visit {
		display: flex;
		flex: 1;
		gap: 8px;
		align-items: center;
		padding: 4px;
		white-space: nowrap;
		overflow: hidden;
		margin: 2px;
		border: 1px dotted var(--border-color);
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
		color: var(--text-color);
	}
	.day_of_month_tile {
		display: flex;
		gap: 4px;
		justify-content: start;
		width: 100%;
	}
	.day_of_month_tile > p {
		margin: 2px;
		color: var(--text-color);
	}

	.month_navigation {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex: 1;
		gap: 8px;
		width: 100%;
		background-color: #6487bd;
		position: relative;
	}

	.small_calendar_header {
		font-size: 20px !important;
	}
	.month_navigation > h1 {
		display: flex;
		flex: 1;
		align-items: center;
		justify-content: center;
		font-family: 'Old Standard TT', serif;
		padding: 0px;
		margin: 0px;
		gap: 8px;
		font-size: 48px;
		color: white;
	}
	.invert {
		background: linear-gradient(90deg, transparent, var(--active-color) 50%) !important;
	}

	.invert-end {
		background: linear-gradient(90deg, var(--active-color) 50%, transparent) !important;
	}
	header {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		text-align: center;
		background-color: transparent;
	}

	header > h3 {
		margin: 0px;
		font-size: medium;
		font-weight: 400;
		font-family: 'Signika', sans-serif;
		color: var(--calendar-background-color);
		border-bottom: 1px solid var(--border-color);
		border-left: 1px solid var(--border-color);
		border-top: none;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: white;
	}

	.month {
		cursor: pointer;
		aspect-ratio: 1;
		display: flex;
		flex: 1;
		padding: 4px;
		min-height: 20px;
		min-width: 20px;
		background-color: var(--calendar-background-color);
		border-bottom: 1px solid var(--border-color);
		border-left: 1px solid var(--border-color);

		flex-direction: column;
	}

	.large_tile {
		min-height: 72px;
		min-width: 72px;
		max-width: 72px;
		max-height: 72px;
	}
	.center {
		justify-content: center;
		align-items: center;
	}

	.month:hover {
		opacity: 0.7;
	}
	.inside {
		background-color: var(--calendar-background-color);
	}
	.active {
		background-color: var(--active-color);
	}

	.left {
		background-color: var(--active-color) !important;
		border-top-left-radius: 50% !important;
		border-bottom-left-radius: 50% !important;
	}
	.right {
		background-color: var(--active-color) !important;
		border-top-right-radius: 50% !important;
		border-bottom-right-radius: 50% !important;
	}
</style>
