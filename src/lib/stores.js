import { writable } from "svelte/store";

export let credits = writable(0);
export let contracts = writable([]);
export let ships = writable([]);
export let agent = writable({});
