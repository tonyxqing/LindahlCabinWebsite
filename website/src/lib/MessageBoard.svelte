<script lang="ts">
	import messages from '$lib/messages.json';
	import { isEmpty } from './Utils';

	let messageInput: string;
</script>

<section>
	<div class="message_container">
		<h5>Message Board</h5>
		<div class="newmessage">
			<textarea placeholder="Write a message to cabin members..." bind:value={messageInput} />
		</div>
		<button
			on:click={() => {
				let date = new Date();
				messages[date.valueOf()] = messageInput;
				messageInput = '';
			}}>Post Message</button
		>
        <div class="message_wrapper">
            {#if !isEmpty(messages)}
			<div>
                {#each Object.entries(messages) as [date, message], i}
                {#if i < 10}
                <div class="message_card">
                    <div class="message_header">
                        <h4>Tony</h4>
                        <p>{new Date(parseInt(date)).getFullYear()}</p>
                    </div>
                    <p class="message">
                        {message}
                    </p>
                </div>
                {/if}
				{/each}
			</div>
            {:else}
			<div>no messages to show</div>
            {/if}
        </div>
    </div>
</section>

<style>
    .message_card {
        height: 'auto';
        display: flex;
        flex-direction: column;
        padding: 12px;
        border: 1px solid black;
        border-radius: 8px;
        margin: 12px;

    }

    .message_header {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 0px;
        margin:0px;
    }
    .message_header > h4 {
        padding: 0px;
        margin:0px;
    }
    .message_header > p{
        font-size: 12px;
    }
    .message {
        padding: 0;
    }
	section {
		display: flex;
		align-items: center;
		padding: 20px;
		flex: 1;
		justify-content: center;
		border-radius: 4px;
	}
	textarea {
		border: none;
		overflow: auto;
		outline: none;
		height: 150px;
		-webkit-box-shadow: none;
		-moz-box-shadow: none;
        background-color: transparent;
		box-shadow: none;
        width: 100%;
        color: white;
        font-size: 1.25rem;
        font-family:'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        resize: none; /*remove the resize handle on the bottom right*/
	}

	.message_container {
		display: flex;
		border: 1px solid black;
        background-color: rgb(67, 52, 97);
		width: 300px;
		height: 500px;
		border-radius: 8px;
		flex-direction: column;
	}
    .message_wrapper {
        
        overflow: auto;
    }
	h5 {
		margin: 0;
	}
	.message_container > div {
	}
</style>
