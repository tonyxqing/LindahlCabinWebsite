<script lang="ts">
	import { onMount } from 'svelte';
	import { getSession } from './client/serverComms';
	import { goto } from '$app/navigation';
	import { account } from './client/authStore';
	import { parseJwt } from './Utils';
	let error_message = '';

	onMount(() => {
		try {
			window.google.accounts.id.initialize({
				client_id: '130478330472-mar4k4d0kea019930om0m7m0elpoju6o.apps.googleusercontent.com',
				callback: async (params: {
					clientId: string;
					client_id: string;
					credential: string;
					select_by: string;
				}) => {
					localStorage.setItem(
						'googleToken',
						JSON.stringify(params)
					);
					let session = await getSession(params.credential);
					switch (session) {
						case 'MAKE_ACCOUNT':
							// make account page
							console.log('make account');
							goto('/signup');
							break;
						default:
							localStorage.setItem('sessionToken', session);
							let token = parseJwt(session);
							$account = token;
							goto('/');
					}
				}
			});
			window.google.accounts.id.renderButton(
				document.getElementById('buttonDiv'),
				{
					type: 'standard',
					size: 'medium',
					theme: 'outline',
					text: 'Sign in with Google',
					shape: 'pill',
					logo_alignment: 'left'
					
				} // customization attributes
			);
		} catch (e) {
			console.log("internet error", e)
			error_message = "Could not load signin button. Please check your internet and try refreshing the page."
		}
	});
</script>

<div id="buttonDiv" />
<div>{error_message}</div>
