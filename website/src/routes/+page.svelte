<script lang="ts">
	import Calendar from '$lib/Calendar.svelte';
	import MessageBoard from '$lib/MessageBoard.svelte';
	import PollBooth from '$lib/PollBooth.svelte';
	import { daysOfWeek } from '$lib/Utils';
	let secondSelectedDate: Date;
	let selectedDate: Date;
</script>

<div>
	<MessageBoard />
	<Calendar />
	<PollBooth />
</div>

<style>
	:global(:root) {
		--main-button: rgb(168, 0, 0);
		--confirm-button: rgba(29, 208, 29, 0.52);
		display: flex;
		justify-content: center;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
			Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
		background-color: #ffffff;
		width: 100%;
	}
	:global(button) {
		background-color: #f9f9f9;
	}
	:global(body) {
		width: 100%;
	}
	div {
		display: flex;
		width: 100%;
		gap: 24px;
	}
	@media (prefers-color-scheme: dark) {
		:global(:root) {
			--main-button: rgb(94, 124, 220);
			--confirm-button: rgb(230, 215, 0);
            --calendar-color: #2a2a2a;
            --active-color: #050508;
			color: #dfe1e2;
			background-color: #02041c;
		}
		a:hover {
			color: #747bff;
		}
		:global(button) {
			--main-button: rgb(94, 124, 220);
		}
		:global(.month) {
			background-color: rgb(47, 47, 47);
		}
	}
</style>
