<script>
	import "../app.css";
	import { agent, credits } from "$lib/stores.js";
	import { fly } from "svelte/transition";
	import { get_agent } from "$lib/helpers/helpers.js";
	import { onMount } from "svelte";

	import Money from "virtual:icons/ri/money-cny-circle-line";
	import Person from "virtual:icons/mdi/badge-account-horizontal";
	import Chevron from "virtual:icons/tabler/arrow-badge-down-filled";

	onMount(async () => {
		get_agent().await;
	});

	export let data;

	let class_base = "mx-auto py-1 px-4  w-full text-center";
	let class_unselected = "hover:bg-cyan-400 hover:text-black " + class_base;
	let class_selected = "bg-cyan-400 text-black " + class_base;
</script>

<div class="flex flex-col h-screen">
	<div class="w-full">
		<div class="p-2 bg-black border-b-2 border-cyan-400">
			<div>
				<h1
					class="text-4xl justify-center mx-auto text-center font-bold border-8 border-cyan-400 p-2"
				>
					Space Traders
				</h1>
				<div class="flex justify-between mt-2">
					{#if $agent != null}
						<span class="flex flex-col">
							<div class="flex">
								<Person class="mr-2 ml-auto my-auto text-xl" />
								{$agent.symbol}
							</div>
							<div class="flex text-start">
								<Chevron
									class="mr-1 ml-auto my-auto text-2xl"
								/>
								{$agent.startingFaction}
							</div>
						</span>
						<span class="flex">
							<span class="m-auto">{$credits}</span>
							<Money class="ml-2 my-auto mr-auto text-xl" /></span
						>
					{/if}
				</div>
			</div>
		</div>
		<div class="flex justify-between bg-black border-b-2 border-cyan-400">
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
			<a
				href="/mission"
				class={data.pathname == "/mission"
					? class_selected
					: class_unselected}
				><ul>Mission</ul>
			</a>
		</div>
	</div>
	{#key data.pathname}
		<div
			in:fly={{ duration: 300, delay: 400, y: 700 }}
			out:fly={{ duration: 300, y: -700 }}
			class="flex-grow overflow-auto"
		>
			<slot />
		</div>
	{/key}
</div>
