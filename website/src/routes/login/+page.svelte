<script>
	import { goto } from '$app/navigation';
	import GoogleSigninButton from '$lib/GoogleSigninButton.svelte';
    import {account} from '$lib/client/authStore'
	import { onMount } from 'svelte';
    $: console.log("THE ACCOUNT FROM THE STORE IS", $account)
	onMount(() => {
		let token = localStorage.getItem('sessionToken')
		if (token) {
			goto('/')
		}
	})
</script>

<div>
	<h3>Lindahl Cabin</h3>
	<GoogleSigninButton />
	<p>
		No account? <a href="signup">Click here to sign up.</a>
	</p>
</div>

<style>
	div {
		display: flex;
		flex: 1;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100%;
	}
</style>
