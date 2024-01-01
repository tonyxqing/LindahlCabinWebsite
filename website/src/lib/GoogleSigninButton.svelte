<script lang="ts">
	import { onMount } from 'svelte';
	import { getSession } from './client/serverComms';
	function parseJwt(token: string) {
		var base64Url = token.split('.')[1];
		var base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
		var jsonPayload = decodeURIComponent(
			window
				.atob(base64)
				.split('')
				.map(function (c) {
					return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
				})
				.join('')
		);

		return JSON.parse(jsonPayload);
	}
	onMount(() => {
		window.google.accounts.id.initialize({
			client_id: '130478330472-mar4k4d0kea019930om0m7m0elpoju6o.apps.googleusercontent.com',
			callback: async (params: {
				clientId: string;
				client_id: string;
				credential: string;
				select_by: string;
			}) => {
				let session = await getSession(params.credential);
				switch (session) {
					case "MAKE_ACCOUNT":
						// make account page
						console.log("make account")
						break;
					case "ACTIVATE_ACCOUNT":
						console.log("activate account")
						break;
					default:
						let token = parseJwt(session);
						console.log(token)

				}
			}
		});
		window.google.accounts.id.renderButton(
			document.getElementById('buttonDiv'),
			{ theme: 'outline', size: 'large' } // customization attributes
		);
	});
</script>

<div id="buttonDiv" />
