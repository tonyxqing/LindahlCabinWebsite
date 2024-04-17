<script lang="ts">
	import { onMount } from 'svelte';
	import { parseJwt } from '$lib/Utils';
	import FaUser from 'svelte-icons/fa/FaUser.svelte';
	import { afterNavigate, goto } from '$app/navigation';
	import { auth } from '$lib/client/serverComms';
	let parsed_token: { profile_pic_url: string, role: "Admin" | "Owner" | "Member" };
	let openDropDownMenu = false;
	let showAdminPanel = false;
	afterNavigate(() => {
		let token = localStorage.getItem('sessionToken');
		if (token) {
			auth.update(() => {
				let jwt = parseJwt(token!);
				return jwt;
			});
			parsed_token = parseJwt(token);
			const {profile_pic_url, role} = parsed_token;
			console.log(profile_pic_url, role)
			if (role === 'Admin') {
				showAdminPanel = true;
			} else {
				showAdminPanel = false;
			}
		} else {
			goto('/login');
		}
	});
</script>

<nav>
	{#if showAdminPanel}
		<a href="/accounts">Accounts</a>
	{/if}
	<a href="/">Home</a>
	<a href="/schedule">Schedule</a>
	<a href="/blog">Blog</a>
	<div>
	<button
		class="menu"
		on:mouseover={() => {
			openDropDownMenu = true;
		}}
		on:focus={() => {
			openDropDownMenu = true;
		}}
	>
		{#if parsed_token}
			<img class="profile_picture" src={parsed_token.profile_pic_url} alt="profile" />
		{:else}
			<div class="profile_picture">
				<FaUser />
			</div>
		{/if}
	</button>
	{#if openDropDownMenu}
		<ul
			class="drop_down_menu"
			on:mouseenter={() => {
				openDropDownMenu = true;
			}}
		>
			<li><a href="/">Home</a></li>
			<li>
				<a
					on:click={() => {
						localStorage.removeItem('sessionToken');
					}}
					href="/login">Sign Out</a
				>
			</li>
		</ul>
	{/if}
	</div>
</nav>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="pane"
	on:mouseenter={() => {
		openDropDownMenu = false;
	}}
>
	<slot />
</div>

<style>
	nav {
		display: flex;
		justify-content: end;
		width: 100%;
		z-index: 10;
		background-color: red;
	}
	nav > a {
		background-color: white;
		display: flex;
		flex: 1;
		justify-content: center;
		align-items: center;
		text-decoration: none;
	}
	nav > a:hover {
		background-color: gray
	}
	button {
		background-color: transparent;
		border: none;
	}
	.pane {
		display: flex;
		flex: 1;
		width: 100%;
	}
	.profile_picture {
		aspect-ratio: 1;
		background-color: transparent;
		border-color: transparent;
		border-radius: 50%;
		color: white;
		height: 40px;
	}

	.drop_down_menu {
		position: absolute;
		list-style-type: none;
		padding: 0;
		margin: 0;
		right: 0;
		z-index: 999;
	}
	.drop_down_menu li:hover {
		background-color: red;
	}
	.drop_down_menu li a {
		background-color: white;
		border: 1px solid black;
		align-items: center;
		text-decoration: none;
		display: flex;
		justify-content: center;
		height: 40px;
		width: 140px;
	}
</style>
