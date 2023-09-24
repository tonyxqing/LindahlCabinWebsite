<script lang="ts">
	import { onMount } from 'svelte';
	import { monthNames, daysOfWeek, numDaysInMonth } from './Utils';
	// Zeller's Congruence formula to get the first day of month
	function getFirstDayOfWeek(month: number, day: number, year: number) {
		if (month < 3) {
			month += 12;
			year -= 1;
		}

		const K = year % 100;
		const J = Math.floor(year / 100);

		const dayOfWeek =
			(day +
				Math.floor((13 * (month + 1)) / 5) +
				K +
				Math.floor(K / 4) +
				Math.floor(J / 4) -
				2 * J) %
			7;
		// + 6 offsets the result to match our daysOfWeek array
		return (dayOfWeek + 6) % 7;
	}

	let date = new Date();

	// The current page on the calendar
	let focusedDay = date.getDate();
	let focusedMonth = date.getMonth();
	let focusedYear = date.getFullYear();
	let selectedDate: Date | null = new Date(`${focusedYear},${focusedMonth + 1},${focusedDay}`);
	let secondSelectedDate: Date | null = null;
	let inputDate: HTMLInputElement;
	let secondInputDate: HTMLInputElement;
	$: isActive = (day: number, offset) => {
		if (selectedDate) {
			let selectedYear = selectedDate.getFullYear();
			let selectedMonth = selectedDate.getMonth();
			let selectedDay = selectedDate.getDate();
			if (secondSelectedDate) {
				let secondSelectedYear = secondSelectedDate.getFullYear();
				let secondSelectedMonth = secondSelectedDate.getMonth();
				let secondSelectedDay = secondSelectedDate.getDate();

				if (
					selectedMonth == (focusedMonth + offset) % 12 &&
					selectedYear == focusedYear &&
					secondSelectedMonth == (focusedMonth + offset) % 12 &&
					secondSelectedYear == focusedYear
				) {
					let predicate = selectedDay <= day + 1 && secondSelectedDay >= day + 1;
					return predicate;
				}

				if (selectedYear <= focusedYear && secondSelectedYear >= focusedYear) {
					const monthOffset = (secondSelectedYear - selectedYear) * 12;
					const focusedMonthOffset = (focusedYear - selectedYear) * 12 + offset;

					if (
						selectedMonth < focusedMonth + focusedMonthOffset &&
						secondSelectedMonth + monthOffset > focusedMonth + focusedMonthOffset
					) {
						return true;
					}

					if (selectedMonth === focusedMonth + focusedMonthOffset) {
						return selectedDay <= day + 1;
					}
					if (secondSelectedMonth === focusedMonth + offset) {
						return secondSelectedDay >= day + 1;
					}
				}
			} else {
				return (
					selectedYear === focusedYear &&
					selectedMonth === focusedMonth + offset &&
					selectedDay === day + 1
				);
			}
		}

		return false;
	};
	$: handleClick = (day, offset) => {
		const clickedDate = new Date(`${focusedYear},${focusedMonth + 1 + offset},${day + 1}`);
		const inputDateString = `${focusedYear}-${(focusedMonth + 1).toString().padStart(2, '0')}-${(
			day + 1
		)
			.toString()
			.padStart(2, '0')}`;
		if (!secondSelectedDate && selectedDate && clickedDate >= selectedDate) {
			secondSelectedDate = clickedDate;
			if (secondInputDate) {
				secondInputDate.value = inputDateString;
			}
		} else {
			secondSelectedDate = null;
			secondInputDate.value = '';

			selectedDate = clickedDate;
			if (inputDate) {
				inputDate.value = inputDateString;
			}
		}
	};
	$: currDays = numDaysInMonth[focusedMonth];
	$: startOffset = getFirstDayOfWeek(focusedMonth + 1, 1, focusedYear);
	$: endOffset = 7 - ((startOffset + currDays) % 7);
	onMount(() => {
		if (inputDate) {
			inputDate.value = `${focusedYear}-${(focusedMonth + 1)
				.toString()
				.padStart(2, '0')}-${focusedDay.toString().padStart(2, '0')}`;
		}
	});
</script>

<section>
	<div class="selected_date_container">
		<div>
			<h1>Arriving</h1>
			<div class="selection_wrapper">
				{#if selectedDate}
					<div class="selected date">
						<p>{selectedDate.getFullYear()}</p>

						<p>{selectedDate.getDate()}</p>

						<p>{monthNames[selectedDate.getMonth()].slice(0, 3).toUpperCase()}</p>
					</div>
				{:else}
					<div class="unselected date">
						<p>----</p>

						<p>--</p>

						<p>---</p>
					</div>
				{/if}
				<input
					bind:this={inputDate}
					type="date"
					on:change={(e) => {
						let inputDate = e.target?.value;
						let date = new Date(inputDate.split('-').join());
						if (!isNaN(date)) {
							focusedMonth = date.getMonth();
							focusedYear = date.getFullYear();
							if (secondSelectedDate && secondSelectedDate < date) {
								secondSelectedDate = null;
								secondInputDate.value = '';
							}
							selectedDate = date;
						}
					}}
				/>
			</div>
		</div>
		<div>
			<h1>Departing</h1>
			<div class="selection_wrapper">
				{#if secondSelectedDate}
					<div class="selected date">
						<p>{secondSelectedDate.getFullYear()}</p>

						<p>{secondSelectedDate.getDate()}</p>

						<p>{monthNames[secondSelectedDate.getMonth()].slice(0, 3).toUpperCase()}</p>
					</div>
				{:else}
					<div class="unselected date">
						<p>----</p>

						<p>--</p>

						<p>---</p>
					</div>
				{/if}
				<input
					bind:this={secondInputDate}
					type="date"
					on:change={(e) => {
						let input = e.target?.value;
						let splitDate = input.split('-');
						let date = new Date(splitDate.join());

						if (!isNaN(date) && splitDate[0] > 1970) {
							if (selectedDate && selectedDate > date) {
								selectedDate = null;
								if (inputDate) {
									inputDate.value = '';
								}
							}
							focusedMonth = date.getMonth();
							focusedYear = date.getFullYear();
							secondSelectedDate = date;
						}
					}}
				/>
			</div>
		</div>
	</div>
	<button
		on:click={() => {
			focusedYear--;
		}}>{`<<`}</button
	>
	<button
		on:click={() => {
			focusedDay = date.getDate();
			focusedMonth = date.getMonth();
			focusedYear = date.getFullYear();
		}}>Today</button
	>
	<button
		on:click={() => {
			focusedYear++;
		}}>{`>>`}</button
	>
	<div class="month_navigation">
		<button
			on:click={() => {
				if (focusedMonth < 1) {
					focusedMonth = 11;
					focusedYear--;
				} else {
					focusedMonth--;
				}
			}}>{`<`}</button
		>
		<h1>{monthNames[focusedMonth] + ' ' + focusedYear}</h1>
		<button
			on:click={() => {
				if (focusedMonth > 10) {
					focusedMonth = 0;
					focusedYear++;
				} else {
					focusedMonth++;
				}
			}}>{`>`}</button
		>
	</div>

	<header>
		{#each daysOfWeek as dayName}
			<h3>{dayName.slice(0, 1)}</h3>
		{/each}
		{#each Array(startOffset).fill(0) as _, day}
			<div
				role="cell"
				tabindex="0"
				on:keyup={() => {}}
				on:click={() =>
					handleClick(numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day, -1)}
				class:active={isActive(
					numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day,
					-1
				)}
				class:month={true}
				class:outside={true}
			>
				{numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day + 1}
			</div>
		{/each}
		{#each Array(currDays).fill(0) as _, day}
			<div
				role="cell"
				tabindex="0"
				on:click={() => handleClick(day, 0)}
				on:keydown={() => {}}
				class="{isActive(day, 0) ? 'active' : 'inside'} month"
			>
				{day + 1}
			</div>
		{/each}
		{#each Array(currDays + startOffset > 34 ? endOffset : endOffset + 7).fill(0) as _, day}
			<div
				role="cell"
				tabindex="0"
				on:click={() => handleClick(day, 1)}
				on:keyup={() => {}}
				class:month={true}
				class:outside={true}
				class:active={isActive(day, 1)}
			>
				{day + 1}
			</div>
		{/each}
	</header>
</section>

<style>
	.month_navigation {
		display: flex;
		justify-content: center;
		align-items: center;
	}
	.month_navigation > h1 {
		width: 250px;
		max-width: 250px;
	}
	.month_navigation > button {
		aspect-ratio: 1;
		border-radius: 50%;
		width: 50px;
		height: 50px;
		background-color: transparent;
		box-shadow: 0px 0px 3px 1px rgb(5, 5, 40);
	}
	.month_navigation > button:hover {
		cursor: pointer;
		box-shadow: 0px 0px 10px 4px rgb(5, 5, 40);
	}
	.selection_wrapper {
		display: flex;
		flex-direction: column;
		gap: 24px;
		align-items: center;
	}
	section {
		background-color: white;
		width: 100%;
		height: 100%;
		flex: 1;
		flex-direction: column;
		align-items: center;
		text-align: center;
		min-width: 700px;
		user-select: none;
	}
	header {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		text-align: center;
		min-width: 50px;
	}

	.month {
		aspect-ratio: 1;
		text-align: start;
		padding: 10px;
		border: 1px solid rgba(138, 97, 97, 0.291);
	}
	.active {
		background-color: rgb(112, 137, 173);
	}
	.month:hover {
		opacity: 0.8;
	}
	.inside {
		background-color: antiquewhite;
	}

	.outside {
		opacity: 0.5;
	}

	.selected_date_container {
		display: flex;
		justify-content: space-around;
		gap: 10px;
	}
	.selected_date_container > div {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.date {
		width: 88px;
		border-radius: 8px;
		z-index: 0;
	}
	.date > p:nth-child(odd) {
		background-color: rgb(168, 0, 0);
		margin: 0px;
		padding: 2px;
		color: white;
		font-weight: 600;
	}
	.date > p:nth-child(even) {
		background-color: #fefef1;
		margin: 0px;
		padding-top: 16px;
		padding-bottom: 16px;
		color: black;
		font-size: 2rem;
	}
	.date > p:first-child {
		border-top-left-radius: inherit;
		border-top-right-radius: inherit;
	}
	.date > p:last-child {
		border-bottom-left-radius: inherit;
		border-bottom-right-radius: inherit;
	}
</style>
