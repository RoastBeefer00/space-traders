<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	let agent;
	async function get_agent() {
		agent = await invoke("get_user_agent", {});
	}

	onMount(async () => {
		get_agent().await;
	});
</script>

<div class="text-2xl flex h-screen">
	<div class="m-auto">
		{#if agent != null}
			<p>Symbol: {agent.symbol}</p>
			<p>Credits: {agent.credits}</p>
			<p>Faction: {agent.startingFaction}</p>
			<p>Ship Count: {agent.shipCount}</p>
		{/if}
	</div>
</div>
