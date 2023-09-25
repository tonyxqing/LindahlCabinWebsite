<script lang="ts">
	import { onMount } from 'svelte';
	import { monthNames, daysOfWeek, numDaysInMonth } from './Utils';
	import { fly } from 'svelte/transition';
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
	let selectingMonth = false;
	let selectingYear = false;

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
			let focusedMonthWithOffset = focusedMonth + offset;
			let yearOffset = 0;
			if (focusedMonthWithOffset < 0) {
				focusedMonthWithOffset = 11;
				yearOffset = -1;
			} else if (focusedMonthWithOffset > 11) {
				focusedMonthWithOffset = 0;
				yearOffset = 1;
			}
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
					let predicate = selectedDay <= day + 1 && secondSelectedDay >= day + 1;
					return predicate;
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
						return true;
					}

					if (selectedMonth === focusedMonthWithOffset) {
						return selectedDay <= day + 1;
					}
					if (secondSelectedMonth === focusedMonthWithOffset) {
						return secondSelectedDay >= day + 1;
					}
				}
			} else {
				return (
					selectedYear === focusedYear + yearOffset &&
					selectedMonth === focusedMonthWithOffset &&
					selectedDay === day + 1
				);
			}
		}

		return false;
	};
	$: handleClick = (day, offset) => {
		let focusedMonthWithOffset = focusedMonth + offset;
		let focusedMonthOffset =
			focusedMonthWithOffset >= 12 ? 0 : focusedMonthWithOffset < 0 ? 11 : focusedMonthWithOffset;
		let yearOffset = focusedMonthWithOffset >= 12 ? 1 : focusedMonthWithOffset < 0 ? -1 : 0;
		const clickedDate = new Date(
			`${focusedYear + yearOffset},${focusedMonthOffset + 1},${day + 1}`
		);
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
	<div>
		<div>
			<div class="month_navigation">
				{#if selectingMonth || selectingYear}
					<div class="button_container">
						{#if selectingMonth}
							{#each monthNames as month, i}
								<button
									class:focused_button={i === focusedMonth}
									on:click={() => {
										(focusedMonth = i), (selectingMonth = false);
									}}>{month.slice(0, 3).toUpperCase()}</button
								>
							{/each}
						{/if}

						{#if selectingYear}
							<button
								on:click={() => {
									focusedYear--;
								}}>{`<`}</button
							>
							{#each Array(8).fill(0) as month, i}
								<button
									class:focused_button={focusedYear - 3 + i === focusedYear}
									on:click={() => {
										(focusedYear = focusedYear - 3 + i), (selectingYear = false);
									}}>{focusedYear - 3 + i}</button
								>
							{/each}
							<button
								on:click={() => {
									focusedYear++;
								}}>{`>`}</button
							>
						{/if}
					</div>
				{:else}
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
						<button
							on:click={() => {
								selectingMonth = true;
							}}>{monthNames[focusedMonth]}</button
						>
						<button
							on:click={() => {
								selectingYear = true;
							}}
						>
							{focusedYear}</button
						>
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
				{/if}
			</div>

			<header>
				{#each daysOfWeek as dayName}
					<h3>{dayName.slice(0, 1)}</h3>
				{/each}
				{#each Array(startOffset).fill(0) as _, day}
					{#if isActive(numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day, -1)}
						{#if day === 0 && isActive(numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day - 1, -1)}
							<span
								role="cell"
								tabindex="0"
								on:keyup={() => {}}
								on:click={() =>
									handleClick(
										numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day,
										-1
									)}
								class:month={true}
								class:invert={true}
								class:outside={true}
							>
								<p>
									{numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day + 1}
								</p>
							</span>
						{:else}
							<span
								role="cell"
								tabindex="0"
								on:keyup={() => {}}
								on:click={() =>
									handleClick(
										numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day,
										-1
									)}
								class:active={true}
								class:month={true}
								class:outside={true}
							>
								<p>
									{numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day + 1}
								</p>
							</span>{/if}
					{:else}
						<div
							role="cell"
							tabindex="0"
							on:keyup={() => {}}
							on:click={() =>
								handleClick(
									numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day,
									-1
								)}
							class:month={true}
							class:outside={true}
						>
							<p>
								{numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11] - startOffset + day + 1}
							</p>
						</div>
					{/if}
				{/each}
				{#each Array(currDays).fill(0) as _, day}
					{#if isActive(day, 0)}
						{#if !startOffset && day === 0 && isActive(numDaysInMonth[focusedMonth ? focusedMonth - 1 : 11], -1)}
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
				{#each Array(currDays + startOffset > 34 ? endOffset : endOffset + 7).fill(0) as _, day}
					{#if isActive(day, 1)}
						{#if day === (currDays + startOffset > 34 ? endOffset - 1 : endOffset + 7 - 1) && isActive(day + 1, 1)}
							<span
								role="cell"
								tabindex="0"
								on:click={() => handleClick(day, 1)}
								on:keydown={() => {}}
								class:month={true}
								class:invert={true}
								class:outside={true}
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
								class:outside={true}
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
							class:outside={true}
						>
							<p>
								{day + 1}
							</p>
						</div>
					{/if}
				{/each}
			</header>
		</div>
		<div class="selected_date_container">
			<div>
				<h1>Arriving</h1>
				<div class="selection_wrapper">
					{#if selectedDate}
						<div
							on:click={() => {
								if (selectedDate) {
									focusedMonth = selectedDate.getMonth();
									focusedYear = selectedDate.getFullYear();
								}
							}}
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
						<div
							on:click={() => {
								if (secondSelectedDate) {
									focusedMonth = secondSelectedDate.getMonth();
									focusedYear = secondSelectedDate.getFullYear();
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
			<div class:button_container={true}>
				<button
					class:disabled={!selectedDate || !secondSelectedDate}
					class:submit={true}
					on:click={() => {
						console.log(selectedDate, secondSelectedDate);
					}}>Schedule Trip</button
				>
			</div>
		</div>
	</div>
</section>

<style>
	.button_container {
		display: flex;
		flex: 1;
		gap: 4px;
		height: 5rem;
		justify-content: center;
		align-items: center;
	}

	.button_container > .disabled {
		opacity: 0.5;
		pointer-events: none;
	}
	.button_container > button {
		flex: 1;
		font-size: 1rem;
		cursor: pointer;
		height: 3rem;
		border: 1px solid var(--main-button);
		background-color: transparent;
		color: var(--main-button);
		border-radius: 8px;
		font-weight: 600;
		max-height: 5rem;
	}
	.button_container > .focused_button {
		border-color: red;
	}
	.button_container > .submit {
		flex: 1;
		font-size: 1rem;
		cursor: pointer;
		border: none;
		color: white;
		border-radius: 8px;
		font-weight: 600;
		max-height: 5rem;
		background-color: var(--confirm-button);
	}
	button:hover {
		opacity: 0.6;
	}
	.month_navigation > h1 > button {
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		background-color: var(--main-button);
		border: none;
		border-radius: 8px;
		font-weight: 600;
		color: white;
		height: 5rem;
		font-size: 2.5rem;
	}
	.month_navigation > h1 > button:first-child {
		flex: 1 1 70%;
		max-width: 20rem;
		min-width: 20rem;
	}
	.month_navigation > h1 > button:last-child {
		flex: 1 1 30%;
	}
	.month_navigation {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex: 1;
		gap: 8px;
		max-width: 30rem;
		min-width: 40rem;
		width: 100%;
	}

	.month_navigation > h1 {
		height: 100%;
		display: flex;
		flex: 1;
		align-items: center;
		justify-content: space-around;
		padding: 0px;
		margin: 0px;
		gap: 8px;
	}

	.month_navigation > button {
		cursor: pointer;
		border: 1px solid var(--main-button);
		background-color: transparent;
		border-radius: 8px;
		font-weight: 800;
		color: var(--main-button);
		width: 5rem;
		height: 5rem;
	}

	.selection_wrapper {
		display: flex;
		flex-direction: column;
		gap: 24px;
		align-items: center;
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
		gap: 24px;
	}
	:global([class~='active']:last-of-type) {
		background-color: rgb(112, 137, 173);
		border-top-right-radius: 60%;
		border-bottom-right-radius: 60%;
	}
	:global([class~='active']:first-of-type) {
		background-color: rgb(112, 137, 173);
		border-top-left-radius: 60%;
		border-bottom-left-radius: 60%;
	}
	:global([class~='active']:last-of-type) > p {
		background-color: rgb(0, 56, 133);
	}
	:global([class~='active']:first-of-type) > p {
		background-color: rgb(0, 56, 133);
	}
	.invert {
		background-color: rgb(112, 137, 173) !important;
	}
	.invert > p {
		background-color: none !important;
		border: none !important;
	}
	header {
		padding-top: 40px;
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		text-align: center;
		background-color: gray;
	}

	header > h3 {
		height: 100%;
		margin: 0px;
	}

	.month {
		cursor: pointer;
		aspect-ratio: 1;
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: transparent;
		min-width: 50px;
		max-width: 100px;
		min-height: 50px;
		max-height: 100px;
	}
	.month > p {
		margin: 0;
		display: flex;
		flex: 1;
		aspect-ratio: 1;
		border-radius: 50%;
		justify-content: center;
		align-items: center;
	}
	.month:hover > p {
		border: 1px solid black;
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
		background-color: rgb(112, 137, 173);
	}

	.inside {
		background-color: transparent;
	}

	.outside {
		opacity: 0.5;
	}

	.selected_date_container {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
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
