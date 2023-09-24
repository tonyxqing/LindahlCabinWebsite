<script lang="ts">
    import Calendar from "$lib/Calendar.svelte";
	import MessageBoard from "$lib/MessageBoard.svelte";
	import PollBooth from "$lib/PollBooth.svelte";
    import { daysOfWeek } from "$lib/Utils";
    let secondSelectedDate: Date;
    let selectedDate: Date;
</script>

<div>
    <MessageBoard/>
    <Calendar/>
    <PollBooth/>
</div>


<style>
    :root {
        display: flex;
        justify-content: center;
        font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
        width: 100%;
    }
    div {
        display: flex;
        width: 100%;
    }
</style>