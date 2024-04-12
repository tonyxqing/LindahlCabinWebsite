<script>
	import GoogleSigninButton from "$lib/GoogleSigninButton.svelte";

    let access_code = ""
</script>
<div>
    <GoogleSigninButton {access_code}/>
    <div>
    <label for="access_code">Code: </label>
        <input type="text" id="access_code" bind:value={access_code}/>
    </div>
    <p>Already have an account? <a href="login">Click here to return to sign in.</a></p>
</div>