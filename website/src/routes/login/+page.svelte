<script>
	import GoogleSigninButton from '$lib/GoogleSigninButton.svelte';
    import {account} from '$lib/client/authStore'
    $: console.log("THE ACCOUNT FROM THE STORE IS", $account)
</script>

<div>
	<GoogleSigninButton />
</div>

<style>
	div {
		display: flex;
		flex: 1;
		justify-content: center;
		align-items: center;
		width: 100%;
	}
</style>
