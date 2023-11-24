<script>
	import "../app.css";
	import { fly } from "svelte/transition";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import credits from "$lib/stores.js";

	import Money from "virtual:icons/ri/money-cny-circle-line";
	import Badge from "virtual:icons/mdi/badge-account-horizontal";
	import Chevron from "virtual:icons/tabler/arrow-badge-down-filled";

	let agent;
	async function get_agent() {
		agent = await invoke("get_user_agent", {});
		credits.set(agent.credits);
	}

	onMount(async () => {
		get_agent().await;
	});

	export let data;

	let class_base = "mx-auto py-1 px-4  w-100";
	let class_unselected = "hover:bg-cyan-400 hover:text-black " + class_base;
	let class_selected = "bg-cyan-400 text-black " + class_base;
</script>

<div>
	<div class="p-2 border-b-2 border-cyan-400 sticky">
		<div>
			<h1
				class="text-4xl justify-center mx-auto text-center font-bold border-8 border-cyan-400 p-2"
			>
				Space Traders
			</h1>
			<div class="flex justify-between mt-2">
				{#if agent != null}
					<span>
						{agent.symbol} -
						{agent.startingFaction}
					</span>
					<span class="flex"
						>{credits}
						<Money class="ml-2 my-auto mr-auto text-xl" /></span
					>
				{/if}
			</div>
		</div>
	</div>
	<div class="flex justify-between border-b-2 border-cyan-400">
		<a
			href="/"
			class={data.pathname == "/" ? class_selected : class_unselected}
			><ul>Agent</ul>
		</a>
		<a
			href="/ships"
			class={data.pathname == "/ships"
				? class_selected
				: class_unselected}
			><ul>Ships</ul>
		</a>
		<a
			href="/contracts"
			class={data.pathname == "/contracts"
				? class_selected
				: class_unselected}
			><ul>Contracts</ul>
		</a>
	</div>
	{#key data.pathname}
		<div
			in:fly={{ duration: 300, delay: 400, y: 700 }}
			out:fly={{ duration: 300, y: -700 }}
		>
			<slot />
		</div>
	{/key}
</div>
