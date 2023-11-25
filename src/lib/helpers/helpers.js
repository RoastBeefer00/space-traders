import { credits, ships } from "$lib/stores.js"
import { invoke } from "@tauri-apps/api/tauri";

export let agent;

export async function get_agent() {
	agent = await invoke("get_user_agent", {});
	credits.set(agent.credits);
}

export async function get_ships() {
	let ships_response = await invoke("get_user_ships", {});
  ships.set(ships_response);
}
