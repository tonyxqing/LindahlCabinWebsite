<script lang="ts">
	import { addBlankMember, getMembers, removeMember } from '$lib/client/serverComms';
	import { onMount } from 'svelte';

	let memberList = getMembers();
	let editing: boolean[] = [];

	onMount(async () => {
		let list = await memberList;
		editing = Array.from(Array(list.length)).fill(false);
	});
</script>

<main>
	<button
		class="admin_add_member_button"
		on:click={async () => {
			await addBlankMember();
			memberList = getMembers();
		}}>Generate New Member</button
	>
	<div class="member_list">
		{#await memberList}
		<div>Loading Users...</div>
		{:then members}
			{#each members as member, i}
				<div class="member_container" class:blank_account={member.accessCode}>
					<h6>Account ID</h6>
					<p>{member.id}</p>
					<div class="member_field">
						<input type="text" value={member.name} class="" disabled={!editing[i]} /><label for=""
							>Name</label
						>
					</div>
					<div class="member_field">
						<input type="text" value={member.email} class="" disabled={!editing[i]} /><label for=""
							>Email</label
						>
					</div>
					<div class="member_field">
						<input type="text" value={member.phone} class="" disabled={!editing[i]} /><label for=""
							>Phone</label
						>
					</div>
					<div class="member_field">
						<input type="text" value={member.sub} class="" disabled={!editing[i]} /><label for=""
							>Subject ID</label
						>
					</div>
					<div class="member_field">
						<input type="text" value={member.accessCode} class="access_code" disabled={!editing[i]} /><label
							for="">Access Code</label
						>
					</div>
					<div>
						<button
							on:click={() => {
								editing[i] = !editing[i];
							}}>Edit</button
						>
						<button
							on:click={async () => {
								await removeMember(member.id);
								memberList = getMembers();
							}}>Delete</button
						>
					</div>
				</div>
			{/each}
		{/await}
	</div>
</main>

<style>
	main {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
		gap: 20px;
		width: 100%;
		height: 100%;
	}
	.admin_add_member_button {
		height: 40px;
		width: 120px;
	}
	.blank_account {
		background-color: var(--background-color) !important;
	}
	.access_code {
		cursor: text;
		font-weight: 800;
	}
	.member_container {
		display: flex;
		flex-direction: column;
		box-shadow: 1px 2px 3px var(--border-color);
		color: var(--text-color);
		height: fit-content;
		background-color: var(--calendar-background-color);
		padding: 8px;
		border-radius: 12px;
	}

	.member_container > h6 {
		font-size: 12px;
		margin: 0px;
	}

	.member_container > p {
		font-size: 12px;
		margin: 0px;
	}
	.member_field {
		display: flex;
		flex-direction: column-reverse;
	}
	.member_list {
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 20px;
		margin: 20px;
	}
	label {
		font-size: 12px;
		font-weight: 800;
		margin-top: 4px;
	}
	input {
		height: 24px;
	}
</style>
