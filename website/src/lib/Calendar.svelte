<script lang="ts">
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import FaArrowRight from 'svelte-icons/fa/FaArrowRight.svelte';
	import { firstDayInMonth, modulus, monthNames, daysOfWeek, numDaysInMonth } from '$lib/client/calendarUtils';

	Date.prototype.day = function () {
		return this.getDate();
	};
	Date.prototype.month = function () {
		return this.getMonth();
	};
	Date.prototype.year = function () {
		return this.getFullYear();
	};
	Date.prototype.date = function () {
		return [this.day(), this.month(), this.year()];
	};
	let date = new Date();
	// The current page on the calendar
	let focused = {
		day: date.getDate(),
		month: date.getMonth(),
		year: date.getFullYear()
	};
	let focusedDay = date.day();
	let focusedMonth = date.month();
	let focusedYear = date.year();
	export let selectedDate: Date | null = new Date(focusedYear, focusedMonth, focusedDay);
	export let secondSelectedDate: Date | null = null;
	let inputDate: HTMLInputElement;
	let secondInputDate: HTMLInputElement;
	$: isActive = (day: number, offset: number) => {
		if (selectedDate) {
			// 0 - none
			// 1 - selected date
			// 2 - between selected dates
			let selectedYear = selectedDate.getFullYear();
			let selectedMonth = selectedDate.getMonth();
			let selectedDay = selectedDate.getDate();
			let focusedMonthWithOffset = focusedMonth + (offset % 12);
			let yearOffset = 0;

			if (secondSelectedDate) {
				let secondSelectedYear = secondSelectedDate.getFullYear();
				let secondSelectedMonth = secondSelectedDate.getMonth();
				let secondSelectedDay = secondSelectedDate.getDate();

				if (
					selectedMonth == focusedMonthWithOffset &&
					selectedYear == focusedYear + yearOffset &&
					secondSelectedMonth == focusedMonthWithOffset &&
					secondSelectedYear == focusedYear + yearOffset
				) {
					if (selectedDay < day + 1 && secondSelectedDay > day + 1) {
						return 2;
					} else if (selectedDay === day + 1 || secondSelectedDay === day + 1) {
						return 1;
					} else {
						return 0;
					}
				}

				if (
					selectedYear <= focusedYear + yearOffset &&
					secondSelectedYear >= focusedYear + yearOffset
				) {
					const monthOffset = (secondSelectedYear - selectedYear) * 12;
					const focusedMonthOffset = (focusedYear + yearOffset - selectedYear) * 12;

					if (
						selectedMonth < focusedMonthWithOffset + focusedMonthOffset &&
						secondSelectedMonth + monthOffset > focusedMonthWithOffset + focusedMonthOffset
					) {
						return 2;
					}

					if (selectedMonth === focusedMonthWithOffset + focusedMonthOffset) {
						if (selectedDay < day + 1) {
							return 2;
						} else if (selectedDay == day + 1) {
							return 1;
						}
					}
					if (secondSelectedMonth + monthOffset === focusedMonthWithOffset + focusedMonthOffset) {
						if (secondSelectedDay > day + 1) {
							return 2;
						} else if (secondSelectedDay == day + 1) {
							return 1;
						}
					}
				}
			} else {
				if (
					selectedYear === focusedYear + yearOffset &&
					selectedMonth === focusedMonthWithOffset &&
					selectedDay === day + 1
				) {
					return 1;
				}
			}
		}

		return 0;
	};
	$: handleClick = (day: number, offset: number) => {
		let focusedMonthWithOffset = focusedMonth + offset;
		let focusedMonthOffset =
			focusedMonthWithOffset >= 12 ? 0 : focusedMonthWithOffset < 0 ? 11 : focusedMonthWithOffset;
		let yearOffset = focusedMonthWithOffset >= 12 ? 1 : focusedMonthWithOffset < 0 ? -1 : 0;
		const clickedDate = new Date(focusedYear + yearOffset, focusedMonthOffset, day + 1);
		const inputDateString = `${focusedYear + yearOffset}-${(focusedMonthOffset + 1)
			.toString()
			.padStart(2, '0')}-${(day + 1).toString().padStart(2, '0')}`;
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
	function getMonthDays(focusedMonth: number) {
		return numDaysInMonth(focusedYear)[
			focusedMonth >= 12 ? focusedMonth - 12 : focusedMonth < 0 ? focusedMonth + 12 : focusedMonth
		];
	}
	function startOffset(focusedMonth: number) {
		return firstDayInMonth(
			focusedMonth >= 12 ? 0 : focusedMonth < 0 ? 11 : focusedMonth + 1,
			1,
			focusedYear
		);
	}
	function endOffset(focusedMonth: number) {
		return 7 - ((startOffset(focusedMonth) + getMonthDays(focusedMonth)) % 7);
	}
	onMount(() => {
		if (inputDate) {
			inputDate.value = `${focusedYear}-${(focusedMonth + 1)
				.toString()
				.padStart(2, '0')}-${focusedDay.toString().padStart(2, '0')}`;
		}
	});
</script>

<section>
	<div>
		<div class="selected_date_container">
			<div>
				<div class="selection_wrapper">
					{#if selectedDate}
						<div
							tabindex="0"
							role="button"
							on:click={() => {
								if (selectedDate) {
									focusedMonth = selectedDate.getMonth();
									focusedYear = selectedDate.getFullYear();
								}
							}}
							on:keypress={() => {}}
							class="selected date"
						>
							<p>{monthNames[selectedDate.getMonth()].slice(0, 3).toUpperCase()}</p>

							<p>{selectedDate.getDate()}</p>

							<p>{selectedDate.getFullYear()}</p>
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
							let splitDate = inputDate.split('-');

							let date = new Date(splitDate[0], splitDate[1] - 1, splitDate[2]);
							if (!isNaN(date) && splitDate[0] > 1970) {
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
			<div class="interval_arrow selection_wrapper">
				<FaArrowRight />
			</div>
			<div>
				<div class="selection_wrapper">
					{#if secondSelectedDate}
						<div
							role="button"
							tabindex="0"
							on:keypress={() => {}}
							on:click={() => {
								if (secondSelectedDate) {
									if (secondSelectedDate.getMonth() === 0) {
										focusedMonth = 11;
										focusedYear = secondSelectedDate.getFullYear() - 1;
									} else {
										focusedMonth = secondSelectedDate.getMonth() - 1;
										focusedYear = secondSelectedDate.getFullYear();
									}
								}
							}}
							class="selected date"
						>
							<p>{monthNames[secondSelectedDate.getMonth()].slice(0, 3).toUpperCase()}</p>

							<p>{secondSelectedDate.getDate()}</p>

							<p>{secondSelectedDate.getFullYear()}</p>
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
							let date = new Date(splitDate[0], splitDate[1] - 1, splitDate[2]);

							if (!isNaN(date) && splitDate[0] > 1970) {
								if (selectedDate && selectedDate > date) {
									selectedDate = null;
									if (inputDate) {
										inputDate.value = '';
									}
								}
								if (date.getMonth() === 0) {
									focusedMonth = 11;
									focusedYear = date.getFullYear() - 1;
								} else {
									focusedMonth = date.getMonth() - 1;
									focusedYear = date.getFullYear();
								}

								secondSelectedDate = date;
							}
						}}
					/>
				</div>
			</div>
		</div>
		<div>
			<div class="calendar_container">
				<div class="calendar_wrapper">
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
						<h1>
							{monthNames[focusedMonth] + ' ' + focusedYear}
						</h1>
					</div>
					<header>
						{#each daysOfWeek as dayName}
							<h3>{dayName.slice(0, 1)}</h3>
						{/each}
						{#each Array(startOffset(focusedMonth)).fill(0) as _, day}
							<div class:month={true} class:outside={true} />
						{/each}
						{#each Array(getMonthDays(focusedMonth)).fill(0) as _, day}
							{#if isActive(day, 0) > 0}
								{#if day === 0 && isActive(day, 0) === 2}
									<span
										role="cell"
										tabindex="0"
										on:click={() => handleClick(day, 0)}
										on:keydown={() => {}}
										class:month={true}
										class:invert={true}
									>
										<p>
											{day + 1}
										</p>
									</span>
								{:else if day === getMonthDays(focusedMonth) - 1 && isActive(day, 0) === 2}
									<span
										role="cell"
										tabindex="0"
										on:click={() => handleClick(day, 0)}
										on:keydown={() => {}}
										class:month={true}
										class:invert-end={true}
									>
										<p>
											{day + 1}
										</p>
									</span>
								{:else}
									<span
										role="cell"
										tabindex="0"
										on:click={() => handleClick(day, 0)}
										on:keydown={() => {}}
										class:month={true}
										class:active={true}
									>
										<p>
											{day + 1}
										</p>
									</span>
								{/if}
							{:else}
								<div
									role="cell"
									tabindex="0"
									on:click={() => handleClick(day, 0)}
									on:keydown={() => {}}
									class:month={true}
									class:inside={true}
								>
									<p>
										{day + 1}
									</p>
								</div>
							{/if}
						{/each}
						{#each Array(getMonthDays(focusedMonth) + startOffset(focusedMonth) > 34 ? endOffset(focusedMonth) : endOffset(focusedMonth) + 7).fill(0) as _, day}
							<div class:month={true} class:outside={true} />
						{/each}
					</header>
				</div>
				<div class="calendar_wrapper">
					<div class="month_navigation">
						<h1>
							{monthNames[(focusedMonth + 1) % 12] +
								' ' +
								(focusedMonth === 11 ? focusedYear + 1 : focusedYear)}
						</h1>
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
						{#each Array(startOffset(focusedMonth + 1)).fill(0) as _, day}
							<div class:month={true} class:outside={true} />
						{/each}
						{#each Array(getMonthDays(focusedMonth + 1)).fill(0) as _, day}
							{#if isActive(day, 1)}
								{#if day === 0 && isActive(day, 1) === 2}
									<span
										role="cell"
										tabindex="0"
										on:click={() => handleClick(day, 1)}
										on:keydown={() => {}}
										class:month={true}
										class:invert={true}
									>
										<p>
											{day + 1}
										</p>
									</span>
								{:else if day === getMonthDays(focusedMonth + 1) - 1 && isActive(day, 1) === 2}
									<span
										role="cell"
										tabindex="0"
										on:click={() => handleClick(day, 1)}
										on:keydown={() => {}}
										class:month={true}
										class:invert-end={true}
									>
										<p>
											{day + 1}
										</p>
									</span>
								{:else}
									<span
										role="cell"
										tabindex="0"
										on:click={() => handleClick(day, 1)}
										on:keydown={() => {}}
										class:month={true}
										class:active={true}
									>
										<p>
											{day + 1}
										</p>
									</span>
								{/if}
							{:else}
								<div
									role="cell"
									tabindex="0"
									on:click={() => handleClick(day, 1)}
									on:keydown={() => {}}
									class:month={true}
									class:inside={true}
								>
									<p>
										{day + 1}
									</p>
								</div>
							{/if}
						{/each}
						{#each Array(getMonthDays(focusedMonth + 1) + startOffset(focusedMonth + 1) > 34 ? endOffset(focusedMonth + 1) : endOffset(focusedMonth + 1) + 7).fill(0) as _, day}
							<div class:month={true} class:outside={true} />
						{/each}
					</header>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.outside {
		pointer-events: none;
	}
	.calendar_container {
		display: flex;
		justify-content: center;
		background-color: #7e8594;
		gap: 16px;
	}
	.calendar_wrapper {
		display: flex;
		justify-content: center;
		flex-direction: column;
	}

	button:hover {
		opacity: 0.6;
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

	.month_navigation > button {
		cursor: pointer;
		position: absolute;
		border: 1px solid var(--main-button);
		background-color: transparent;
		border-radius: 50%;
		aspect-ratio: 1;
		font-weight: 800;
		color: var(--main-button);
	}
	.month_navigation > button:last-child {
		right: 0;
	}
	.selection_wrapper {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
	}
	section {
		background-color: transparent;
		width: 100%;
		height: 100%;
		flex: 1;
		flex-direction: column;
		align-items: center;
		text-align: center;
		user-select: none;
	}
	section > div {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}
	:global([class~='active']:last-of-type) {
		background-color: rgb(112, 137, 173);
		border-top-right-radius: 50%;
		border-bottom-right-radius: 50%;
	}
	:global([class~='active']:first-of-type) {
		background-color: rgb(112, 137, 173);
		border-top-left-radius: 50%;
		border-bottom-left-radius: 50%;
	}
	:global([class~='active']:last-of-type) > p {
		background-color: var(--main-button);
	}
	:global([class~='active']:first-of-type) > p {
		background-color: var(--main-button);
	}
	.invert {
		background: linear-gradient(90deg, transparent, var(--active-color) 50%) !important;
	}
	.invert > p {
		background-color: none !important;
		border: none !important;
	}

	.invert-end {
		background: linear-gradient(90deg, var(--active-color) 50%, transparent) !important;
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
	.interval_arrow {
		fill: white;
		width: 35px;
		padding-bottom: 25px;
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
	.active {
		background-color: var(--active-color);
	}

	.inside {
		background-color: transparent;
	}

	.selected_date_container {
		display: flex;
		justify-content: center;
		background-color: #595e5b;
		gap: 12px;
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
		background-color: var(--main-button);
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
