<script lang="ts">
	import { afterNavigate, goto } from '$app/navigation';
	import { registerMember } from '$lib/client/serverComms';
	import { onMount } from 'svelte';

	let access_code = '';
	let params: {
		clientId: string;
		client_id: string;
		credential: string;
		select_by: string;
	};
	let error_message: string;
	onMount(() => {
		let googleToken = localStorage.getItem('googleToken');
		if (googleToken) {
			params = JSON.parse(googleToken);
		} else {
			goto('/');
		}
	});
</script>

<main>
	<div class="access_code_container">
		<label for="access_code">Code:</label>
		<input type="text" id="access_code" bind:value={access_code} />
	</div>
	<button
		on:click={async () => {
			let update = await registerMember(access_code, params.credential);
			localStorage.setItem('sessionToken', update.data.registerMember);
			if (update.errors) {
				error_message = update().errors[0].message;
			}
			localStorage.removeItem('googleToken');
			window.location.href = '/';
		}}>
		Sign Up
	</button>
</main>

<style>
	.access_code_container {
		display: flex;
		gap: 8px;
	}

	input {
		height: 28px;
		font-size: 18px;
	}
	main {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 1;
	}
</style>
