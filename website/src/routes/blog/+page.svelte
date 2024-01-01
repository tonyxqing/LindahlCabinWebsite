<script lang="ts">
	import {
		addMessage,
		getMessages,
		removeMessage,
		type Message,
		addComment,

		removeComment

	} from '$lib/client/serverComms';
	import { onMount } from 'svelte';
	import FaTrash from 'svelte-icons/fa/FaTrash.svelte';
	import FaRegThumbsUp from 'svelte-icons/fa/FaRegThumbsUp.svelte';
	import FaRegSmile from 'svelte-icons/fa/FaRegSmile.svelte';
	import FaRegHeart from 'svelte-icons/fa/FaRegHeart.svelte';
	let messages: Message[] = [];
	onMount(async () => {
		messages = await getMessages();
	});
	let text = '';
	let comment: string[] = [];
</script>

<p>All Blog Posts</p>
<textarea bind:value={text} />
<button
	on:click={async () => {
		await addMessage('foop', text);
		text = '';
		messages = await getMessages();
	}}>Post</button
>
{#each messages as { id, creatorId, content, postedOn, reactions, comments }, i}
	<div class="message_container">
		<button
			class="delete_button"
			on:click={async () => {
				await removeMessage(id);
				messages = await getMessages();
			}}><FaTrash /></button
		>
		<div>
			{creatorId}
		</div>
		<p>
			{content}
		</p>
		<sub>
			{postedOn}
		</sub>
		<div class="response_container">
			<div class="reactions_container">
				<div class="heart_icon">
					<FaRegHeart />
				</div>
				{reactions.filter((x) => x == 'HEART').length}
				<div class="smile_icon">
					<FaRegSmile />
				</div>
				{reactions.filter((x) => x == 'HEART').length}

				<div class="thumbs_up_icon">
					<FaRegThumbsUp />
				</div>
				{reactions.filter((x) => x == 'HEART').length}
			</div>
			<input type="text" bind:value={comment[i]} /><button
				on:click={async () => {
					await addComment(id, comment[i]);
					messages = await getMessages();
				}}>Comment</button
			>
			{#each comments as { id: commentId, creatorId, content, reactions }}
				<div>
					<button
						class="delete_button"
						on:click={async () => {
							await removeComment(id, commentId);
							messages = await getMessages();
						}}><FaTrash /></button
					>
					{creatorId}
					{content}
				</div>
			{/each}
		</div>
	</div>
{/each}

<style>
	.heart_icon {
		color: red;
	}

	.smile_icon {
		color: rgb(175, 175, 66);
	}

	.thumbs_up_icon {
		color: rgb(60, 150, 150);
	}
	.message_container {
		position: relative;
		margin: 4px auto;
		background-color: aliceblue;
		width: 480px;
		height: 240px;
		min-height: 150px;
		padding: 12px;
		border-radius: 8px;
	}
	.delete_button {
		float: right;
		height: 20px;
	}
	.response_container {
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
	}
</style>
