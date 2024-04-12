import { writable } from "svelte/store";

export const account = writable({
  profile_pic_url: null,
  foop: null,
  is_admin: null,
  is_owner: null,
});
