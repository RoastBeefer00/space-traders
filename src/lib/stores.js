import { writable } from "svelte/store";

export let credits = writable(0);
export let ships = writable([]);
