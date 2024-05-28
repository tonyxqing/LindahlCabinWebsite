<script lang="ts">
	import {
		addMessage,
		getMessages,
		removeMessage,
		type Message,
		addComment,
		removeComment,
		auth
	} from '$lib/client/serverComms';
	import { onMount } from 'svelte';

	import MessageBoard from '$lib/MessageBoard.svelte';
	let messages: Promise<Message[]> = getMessages();
	let text = '';
	$: console.log($auth);
	function updateMessages () {
		messages = getMessages();
	}
</script>

<main>
	<div class="message_post_container">
		<div class="message_header">
			<img class="main_post_profile_pic" src={$auth.profile_pic_url} alt="poster's avatar" />
			<p>{$auth.name}</p>
		</div>
		<textarea placeholder="Enter message here..." bind:value={text} />
		<button
			class="post_button"
			on:click={async () => {
				await addMessage(text);
				text = '';
				updateMessages();
			}}>Post</button
		>
	</div>
	<h3>All Blog Posts</h3>
	<div class="message_container">
		{#await messages}
		<div>Loading posts...</div>
		{:then messages}
		{#each messages as message, i}
			<MessageBoard {message} {updateMessages}/>
		{/each}
		{/await}
	</div>
</main>

<style>
	main {
		margin: auto;
	}

	.main_post_profile_pic {
		height: 25px;
		aspect-ratio: 1;
		border-radius: 50%;
	}

	p {
		margin: 0;
	}
	.post_button {
		height: 24px;
		width: 50px;
		align-self: end;
		background-color: var(--background-color);
		color: var(--text-color);
		border: 1px solid var(--text-color);
		border-radius: 4px;
	}


	.message_header {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.message_header > p {
		font-size: medium;
		font-weight: 600;
	}
	.message_post_container {
		background-color: var(--calendar-background-color);
		margin: 4px auto;
		border-radius: 8px;
		padding: 12px;
		display: flex;
		flex-direction: column;
		color: var(--text-color);
	}
	.message_post_container > textarea {
		margin: 8px 0px;
		border-radius: 4px;
		background-color: transparent;
		color: var(--text-color) !important;
		font-family: 'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
		outline: none;
		border: none;
		padding: 0;
		resize: none;
	}

	.message_container {
		display: flex;
		flex-direction: column-reverse;
	}

	/* .response_container {
		position: absolute;
		bottom: 24px;
		left: 8px;
	}
	.reactions_container {
		display: flex;
		gap: 16px;
		height: 20px;
	}

	.message_container > sub {
		position: absolute;
		bottom: 8px;
		left: 8px;
	} */
</style>
