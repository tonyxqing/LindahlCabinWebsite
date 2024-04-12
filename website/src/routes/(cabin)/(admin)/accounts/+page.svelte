<script>
	import { addBlankMember, getMembers, removeMember } from "$lib/client/serverComms";

    let memberList = getMembers();
    let editing = false; 
</script>

<button on:click={async () => {await addBlankMember(); memberList = getMembers()}}>Add Member</button>
{#await memberList then members}
{#each members as member}
    <div class="member_container">
        <p>{member.id}</p>
        <div class="member_field"><input type="text" value={member.sub} class=""><label for="">Sub</label></div>
        <div class="member_field"><input type="text" value={member.name} class=""><label for="">Name</label></div>
        <div class="member_field"><input type="text" value={member.email} class=""><label for="">Email</label></div>
        <div class="member_field"><input type="text" value={member.phone} class=""><label for="">Phone</label></div>
        <div class="member_field"><input type="text" value={member.accessCode} class=""><label for="">Access Code</label></div>
        <button>Edit</button>
        <button on:click={async () => {await removeMember(member.id); memberList = getMembers()}}>Delete</button>
    </div>
{/each}
{/await}

<style>
    .member_container {
        border: 1px solid black;
    }
    .member_field {
        display: flex; 
        flex-direction: column-reverse;
    }
</style>
