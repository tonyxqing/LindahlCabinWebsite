<script lang="ts">
	import {
		getMessages,
		removeMessage,
		type Message,
		addComment,
		removeComment,
		auth
	} from '$lib/client/serverComms';
	import FaTrash from 'svelte-icons/fa/FaTrash.svelte';
	import FaRegComment from 'svelte-icons/fa/FaRegComment.svelte';
	import FaPaperPlane from 'svelte-icons/fa/FaPaperPlane.svelte';
	import { computeElapsedString } from './Utils';
	export let message: Message;
	export let updateMessages;
	let commenting: boolean = false;
	let { id, creatorId, content, postedOn, comments, name, profilePic } = message;
	let commentContent: string = '';
	const posted = (postedOn: string) => {
		const cleanedDateString = postedOn.replace(/:\d{2}$/, '');
		const past = new Date(cleanedDateString);
		const now = new Date();
		return computeElapsedString(now, past);
	};
</script>

<div class="message">
	{#if creatorId === $auth.sub}
		<button
			class="delete_button"
			on:click={async () => {
				if (confirm('would you like to delete this message?')) {
					await removeMessage(id);
					updateMessages();
				}
			}}><FaTrash /></button
		>
	{/if}
	<div>
		<div class="message_header">
			<img class="main_post_profile_pic" src={profilePic} alt="poster's avatar" />
			<p>{name}</p>
			<sub>
				{posted(postedOn)}
			</sub>
		</div>
	</div>
	<p class="message_content">
		{content}
	</p>
	<div class="comment_bar">
		<button
			class="comment_icon"
			on:click={() => {
				commenting = !commenting;
			}}
		>
			<FaRegComment />
			Comment
		</button>
	</div>
	<div class="comment_container">
		{#each comments as { id: commentId, creatorId, content, name, profilePic, postedOn }}
			<div class="comment">
				<div class="comment">
					<div class="comment_wrapper">
						<div class="comment">
							<img class="comment_profile_pic" src={profilePic} alt="commenter's avatar" />
							<h5>{name}:</h5>
						</div>
						<sub>{posted(postedOn)}</sub>
					</div>
					<p class="comment_content">{content}</p>
				</div>
				{#if creatorId === $auth.sub}
					<button
						class="delete_button"
						on:click={async () => {
							if (confirm('Would you like to delete this comment?')) {
								await removeComment(id, commentId);
								updateMessages();
							}
						}}><FaTrash /></button
					>
				{/if}
			</div>
		{/each}
	</div>
	{#if commenting}
		<div class="comment_post_container">
			<input placeholder="Enter your comment here..." type="text" bind:value={commentContent} />
			<button
				class="send_button_icon"
				on:click={async () => {
					await addComment(id, commentContent);
					updateMessages();
				}}><FaPaperPlane /></button
			>
		</div>
	{/if}
</div>

<style>
	.comment_container {
		display: flex;
		flex-direction: column;
		margin: 4px;
	}

	.comment_wrapper {
		display: flex;
		flex-direction: column;
		text-wrap: nowrap;
		align-self: flex-start;
		min-width: 150px;
		max-width: 300px;
	}
	.comment_content {
		display: flex;
		align-self: flex-start;
		margin: 2px;
	}
	.comment_container > .comment:hover {
		border: 1px dotted rgba(213,124,123,0.7)
	}

	.comment_container > .comment {
		border: 1px dotted transparent;
	}
	.message {
		position: relative;
		background-color: var(--calendar-background-color);
		width: 480px;
		border-radius: 8px;
		color: var(--text-color);
		margin: 8px;
	}

	.send_button_icon {
		height: 18px;
		padding: 0;
		background-color: transparent;
		border: none;
		color: var(--text-color);
	}
	.message_content {
		margin: 12px;
	}

	.comment_post_container {
		display: flex;
		gap: 8px;
		align-items: center;
		margin: 12px;
	}
	.comment_post_container > input {
		background-color: var(--text-color);
		border-radius: 12px;
		color: var(--background-color);
		border: none;
		outline: none;
		height: 28px;
		padding: 4px 8px;
		font-size: 14px;

	}

	.comment_icon {
		cursor: pointer;
		color: var(--text-color);
		background-color: transparent;
		border: none;
		height: 28px;
		padding: 0;
		display: flex;
		align-items: center;
		gap: 8px;
		font-weight: 600;
	}
	.delete_button {
		float: right;
		height: 18px;
		min-width: 18px;
		margin: 2px;
		margin-top: 4px;
		cursor: pointer;
		color: var(--text-color);
		background-color: transparent;
		border: none;
		padding: 0;
		align-self: flex-start;
	}
	h5 {
		margin: 0;
	}
	.comment {
		display: flex;
		gap: 4px;
		align-items: center;
		margin: 4px;
		justify-content: flex-start;
	}
	.comment_profile_pic {
		height: 16px;
		aspect-ratio: 1;
		border-radius: 50%;
	}
	.comment_bar {
		display: flex;
		height: 28px;
		border-top: 1px dotted var(--text-color);
		border-bottom: 1px dotted var(--text-color);
		padding: 8px 12px;
	}

	.main_post_profile_pic {
		height: 25px;
		aspect-ratio: 1;
		border-radius: 50%;
	}

	.main_post_profile_pic {
		height: 25px;
		aspect-ratio: 1;
		border-radius: 50%;
	}

	p {
		margin: 0;
	}

	.message_header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin: 12px;
	}

	.message_header > p {
		font-size: medium;
		font-weight: 600;
	}

	sub {
		margin: 0;
		font-size: x-small;
		align-self: flex-start;
		margin-right: 4px;
	}
</style>
