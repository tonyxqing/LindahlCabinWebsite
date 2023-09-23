<script lang="ts">
    import Calendar from "$lib/Calendar.svelte";
    let secondSelectedDate:Date;
    let selectedDate:Date;
    $: console.log(selectedDate, secondSelectedDate)
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<h3>Embarkment</h3>
<p>{selectedDate}</p>
<h3>Disembarkment</h3>
<p>{secondSelectedDate}</p>
<div>
    <Calendar bind:selectedDate={selectedDate} bind:secondSelectedDate={secondSelectedDate}/>
</div>


<style>
    :root {
        display: flex;
        justify-content: center;
    }
    div {
    }
</style>