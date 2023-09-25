<script>
	import { isEmpty } from "./Utils";

	let polls = {};
</script>

<section>
    <div class="poll_container">
        <div>
            <p></p>
            <input type="range"/>
        </div>
        {#if !isEmpty(polls)}
            <div>
                <p></p>
                <input type="range"/>
            </div>
        {/if}
		<div>no-polls to show</div>
	</div>
</section>

<style>
	section {
		display: flex;
		align-items: center;
		padding: 20px;
		flex: 1;
		border-radius: 4p;
	}
	.poll_container {
		display: flex;
		border: 1px solid black;
		width: 300px;
		height: 500px;
		border-radius: 8px;
	}
</style>
