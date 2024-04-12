<script lang="ts">
	import { onMount } from 'svelte';
	import { getSession, registerMember, updateMember } from './client/serverComms';
	import { goto } from '$app/navigation';
	import { account } from './client/authStore';
	import { parseJwt } from './Utils';
	export let access_code: string = '';
	let error_message = '';

	onMount(() => {
		window.google.accounts.id.initialize({
			client_id: '130478330472-mar4k4d0kea019930om0m7m0elpoju6o.apps.googleusercontent.com',
			callback: async (params: {
				clientId: string;
				client_id: string;
				credential: string;
				select_by: string;
			}) => {
				if (!access_code) {
					let session = await getSession(params.credential);
					switch (session) {
						case 'MAKE_ACCOUNT':
							// make account page
							console.log('make account');
							goto('/signup');
							break;
						default:
							console.log(session);
							localStorage.setItem('sessionToken', session);
							let token = parseJwt(session);
							console.log('token is', token);
							$account = token;
							goto('/')
					}
				} else {
					let update = await registerMember(access_code, params.credential);
					console.log(update)
					if (update.errors) {
						error_message = update.errors[0].message
					}

					if (update.registerMember && update.registerMember.id) {
						goto('/')
					}
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
	});
</script>

<div id="buttonDiv" />
<div>{error_message}</div>
