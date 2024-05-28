<script lang="ts">
	import { parseJwt } from '$lib/Utils';
	import FaUser from 'svelte-icons/fa/FaUser.svelte';
	import { afterNavigate, goto } from '$app/navigation';
	import { auth } from '$lib/client/serverComms';
	let parsed_token: { profile_pic_url: string; role: 'Admin' | 'Owner' | 'Member' };
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
			const { profile_pic_url, role } = parsed_token;
			console.log(profile_pic_url, role);
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

<section>
	<nav>
		{#if showAdminPanel}
			<a href="/accounts">ACCOUNTS</a>
		{/if}
		<a href="/">HOME</a>
		<a href="/schedule">SCHEDULE</a>
		<a href="/blog">BLOG</a>
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
</section>

<style>
	nav {
		display: flex;
		position: absolute;
		justify-content: end;
		width: 80%;
		align-self: center;
		padding: 16px 0;
		z-index: 10;
	}
	nav > a {
		background-color: rgba(0, 0, 0, 0.1);
		opacity: 1;
		font-weight: 600;
		font-size: 20px;
		color: #b9c5d6;
		display: flex;
		flex: 1;
		justify-content: center;
		align-items: center;
		text-decoration: none;
	}
	nav > a:hover {
		background-color: #6388be;
		color: white;
		opacity: 0.8;
	}
	button {
		background-color: transparent;
		border: none;
	}
	section {
		display: flex;
		flex: 1;
		flex-direction: column;
		width: 100%;
		height: 100vh;
		overflow: hidden;
	}

	.pane {
		display: flex;
		flex: 1;
	}
	.profile_picture {
		aspect-ratio: 1;
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

	.menu {
		background-color: rgba(0, 0, 0, 0.1);
	}

	.menu:hover {
		background-color: rgba(99,136,190, 0.8);
		color: white;
		cursor: pointer;
	}

	.drop_down_menu li:hover {
		background-color: rgba(99,136,190, 0.8);

	}
	.drop_down_menu li a {
		background-color: rgba(0, 0, 0, 0.1);

		color: white;
		align-items: center;
		text-decoration: none;
		display: flex;
		justify-content: center;
		height: 40px;
		width: 140px;
	}
</style>
