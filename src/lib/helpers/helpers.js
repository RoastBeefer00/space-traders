import { agent, credits, ships, contracts } from "$lib/stores.js";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";

// export let agent;

export async function get_agent() {
	agent.set(await invoke("get_user_agent", {}));
	credits.set(get(agent).credits);
}

export async function get_ships() {
	let ships_response = await invoke("get_user_ships", {});
	ships.set(ships_response);
}

export async function get_contracts() {
	let contracts_response = await invoke("get_user_contracts", {});
	contracts.set(contracts_response);
}
