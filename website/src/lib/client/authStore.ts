import { writable } from 'svelte/store';

export const account = writable({
	foop: null,
	is_admin: null,
	is_owner: null
});
