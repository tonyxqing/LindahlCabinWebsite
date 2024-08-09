<script>
	import { goto } from '$app/navigation';
	import GoogleSigninButton from '$lib/GoogleSigninButton.svelte';
	import { onMount } from 'svelte';

	
	onMount(() => {
		let token = localStorage.getItem('sessionToken');
		if (token) {
			goto('/');
		}
	});
</script>

<div>
	<h1>Login</h1>
	<GoogleSigninButton />
	<p>No account? <a href="/account-request">Request an account here.</a></p>
</div>

<style>
	div {
		margin: auto 0;
		padding: 10px;
		height: 200px;
		width: 340px;
		background-color: var(--text-color);
	}
</style>
