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

<main>
	<div class="message_post_container">
		<h5>Post message</h5>
		<textarea bind:value={text} />
		<button
			on:click={async () => {
				await addMessage(text);
				text = '';
				messages = await getMessages();
			}}>Post</button
		>
	</div>
	<h3>All Blog Posts</h3>
	<div class="message_container">
		{#each messages as { id, creatorId, content, postedOn, reactions, comments, name, profilePic }, i}
			<div class="message">
				<button
					class="delete_button"
					on:click={async () => {
						await removeMessage(id);
						messages = await getMessages();
						console.log(messages);
					}}><FaTrash /></button
				>
				<img class="main_post_profile_pic" src={profilePic} />
				<div>
					<p>{name}</p>
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
							await addComment(id, comment[i], $auth.id);
							messages = await getMessages();
						}}>Comment</button
					>
					<div class="comment_container">
						{#each comments as { id: commentId, creatorId, content, reactions, name, profilePic }}
							<div class="comment">
								<button
									class="delete_button"
									on:click={async () => {
										await removeComment(id, commentId);
										messages = await getMessages();
									}}><FaTrash /></button
								>
								<img class="comment_profile_pic" src={profilePic} />
								{name}
								{creatorId}
								{content}
							</div>
						{/each}
					</div>
				</div>
			</div>
		{/each}
	</div>
</main>

<style>
	main {
		margin: auto;
	}
	.heart_icon {
		color: red;
	}

	.smile_icon {
		color: rgb(175, 175, 66);
	}

	.thumbs_up_icon {
		color: rgb(60, 150, 150);
	}

	.main_post_profile_pic {
		height: 25px;
		aspect-ratio: 1;
		border-radius: 50%;
	}
	.comment_profile_pic {
		height: 16px;
		aspect-ratio: 1;
		border-radius: 50%;
	}
	.message_post_container {
		background-color: aliceblue;
		margin: 4px auto;
		border-radius: 8px;
		padding: 12px;
		display: flex;
		flex-direction: column;
	}
	h5 {
		margin: 0;
	}
	.message_container {
		display: flex;
		flex-direction: column-reverse;
	}
	.comment_container {
		display: flex;
		flex-direction: column-reverse;
	}
	.message {
		position: relative;
		margin: 4px auto;
		background-color: aliceblue;
		width: 480px;
		height: 240px;
		min-height: 150px;
		padding: 12px;
		border-radius: 8px;
		color: black;
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
