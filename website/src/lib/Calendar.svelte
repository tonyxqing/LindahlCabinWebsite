<script>
	let selectedMonth = '0';
	const daysOfWeek = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const monthsOfYear = [
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
	const monthsAndDays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

	let date = new Date();

	let focusedDay = date.getDate();
	let focusedMonth = date.getMonth();
	let focusedYear = date.getFullYear();

	function getFirstDayOfWeek(month, day, year) {
		if (month < 3) {
			month += 12;
			year -= 1;
		}

		const K = year % 100;
		const J = Math.floor(year / 100);

		// Zeller's Congruence formula
		const dayOfWeek =
			(day +
				Math.floor((13 * (month + 1)) / 5) +
				K +
				Math.floor(K / 4) +
				Math.floor(J / 4) -
				2 * J) %
			7;

		// Convert the result to a day of the week

		return (dayOfWeek + 6) % 7;
	}
	$: currDays = monthsAndDays[focusedMonth];
	$: startOffset = getFirstDayOfWeek(focusedMonth + 1, 1, focusedYear);
	$: endOffset = 7 - ((startOffset + currDays) % 7);
</script>

<section>
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
	<div>{monthsOfYear[focusedMonth] + ' ' + focusedYear}</div>
	<header>
		{#each daysOfWeek as dayName}
			<h1>{dayName}</h1>
		{/each}
		{#each Array(startOffset).fill(0) as _, day}
			<div>{monthsAndDays[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day + 1}</div>
		{/each}
		{#each Array(currDays).fill(0) as _, day}
			<div>{day + 1}</div>
		{/each}
		{#each Array(endOffset).fill(0) as _, day}
			<div>{day + 1}</div>
		{/each}
	</header>
</section>

<style>
	section {
		background-color: white;
	}
	header {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		text-align: center;
	}
	header > h1 {
		border: 1px solid black;
	}
</style>
