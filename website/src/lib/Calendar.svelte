<script lang="ts">
	const daysOfWeek = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const monthNames = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	];
	const numDaysInMonth = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
	let isDragging = false;
	export let selectedDate = new Date(
		Date.parse(`${focusedYear},${focusedMonth + 1},${focusedDay}`)
	);
	export let secondSelectedDate: Date | null = null;
	$: isActive = (day: number) => {
		let selectedYear = selectedDate.getFullYear();
		let selectedMonth = selectedDate.getMonth();
		let selectedDay = selectedDate.getDate();
		if (secondSelectedDate) {
			let secondSelectedYear = secondSelectedDate.getFullYear();
			let secondSelectedMonth = secondSelectedDate.getMonth();
			let secondSelectedDay = secondSelectedDate.getDate();

			if (
				selectedMonth == focusedMonth &&
				selectedYear == focusedYear &&
				secondSelectedMonth == focusedMonth &&
				secondSelectedYear == focusedYear
			) {
				let predicate = selectedDay <= day + 1 && secondSelectedDay >= day + 1;
				return predicate;
			}

			if (selectedYear <= focusedYear && secondSelectedYear >= focusedYear) {
				const monthOffset = (secondSelectedYear - selectedYear) * 12;
				const focusedMonthOffset = (focusedYear - selectedYear) * 12;

				if (
					selectedMonth < focusedMonth + focusedMonthOffset &&
					secondSelectedMonth + monthOffset > focusedMonth + focusedMonthOffset
				) {
					return true;
				}

				if (selectedMonth === focusedMonth + focusedMonthOffset) {
					return selectedDay <= day + 1;
				}
				if (secondSelectedMonth === focusedMonth) {
					return secondSelectedDay >= day + 1;
				}
			}
		} else {
			return (
				selectedYear === focusedYear && selectedMonth === focusedMonth && selectedDay === day + 1
			);
		}

		return false;
	};
	$: currDays = numDaysInMonth[focusedMonth];
	$: startOffset = getFirstDayOfWeek(focusedMonth + 1, 1, focusedYear);
	$: endOffset = 7 - ((startOffset + currDays) % 7);
</script>

<section>
	<h1>{monthNames[focusedMonth] + ' ' + focusedYear}</h1>
	<button
		on:click={() => {
			focusedYear--;
		}}>{`<<`}</button
	>
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
	<button
		on:click={() => {
			focusedDay = date.getDate();
			focusedMonth = date.getMonth();
			focusedYear = date.getFullYear();
		}}>Today</button
	>
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
	<button
		on:click={() => {
			focusedYear++;
		}}>{`>>`}</button
	>
	<header>
		{#each daysOfWeek as dayName}
			<h3>{dayName}</h3>
		{/each}
		{#each Array(startOffset).fill(0) as _, day}
			<div class="outside month">
				{numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day + 1}
			</div>
		{/each}
		{#each Array(currDays).fill(0) as _, day}
			<div
				role="cell"
				tabindex="0"
				on:mousedown={() => {
					isDragging = true;
					const clickedDate = new Date(Date.parse(`${focusedYear},${focusedMonth + 1},${day + 1}`));
					if (!secondSelectedDate && clickedDate > selectedDate) {
						secondSelectedDate = clickedDate;
					} else {
						secondSelectedDate = null;
						selectedDate = clickedDate;
					}
				}}
				on:keydown={() => {}}
				on:mouseup={() => {
					isDragging = false;
				}}
				on:mouseenter={() => {
					if (isDragging) {
						const clickedDate = new Date(
							Date.parse(`${focusedYear},${focusedMonth + 1},${day + 1}`)
						);
						if (clickedDate > selectedDate) {
							secondSelectedDate = clickedDate;
						} else {
							secondSelectedDate = null;
							selectedDate = clickedDate;
						}
					}
				}}
				class="{isActive(day) ? 'active' : 'inside'} month"
			>
				{day + 1}
			</div>
		{/each}
		{#each Array(currDays + startOffset > 34 ? endOffset : endOffset + 7).fill(0) as _, day}
			<div class="outside month">{day + 1}</div>
		{/each}
	</header>
</section>

<style>
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
		min-width: 70px;
	}

	.month {
		aspect-ratio: 1;
		text-align: start;
		padding: 10px;
		border: 1px solid rgb(138, 97, 97);
	}
	.active {
		background-color: rgb(112, 137, 173);
	}
	.inside:hover {
		opacity: 0.6;
	}
	.inside {
		background-color: antiquewhite;
	}

	.outside {
		background-color: whitesmoke;
		opacity: 0.4;
	}
</style>
