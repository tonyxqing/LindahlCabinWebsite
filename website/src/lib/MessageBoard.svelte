<script>
	import messages from '$lib/messages.json';
	function isEmpty(obj) {
		for (const prop in obj) {
			if (Object.hasOwn(obj, prop)) {
				return false;
			}
		}

		return true;
	}
</script>

<section>
	{#if !isEmpty(messages)}
		<div>
			{#each messages as message}{/each}
		</div>
	{:else}
		<div>no messages to show</div>
	{/if}
</section>

<style>
    section {
        display: flex;
        align-items: center;
        padding: 20px;
        flex: 1;
    }
</style>