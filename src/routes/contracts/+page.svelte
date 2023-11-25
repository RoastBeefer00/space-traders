<script>
	import { onMount } from "svelte";
	import { get_contracts } from "$lib/helpers/helpers.js";
	import { contracts } from "$lib/stores.js";
	import Contract from "../../lib/contract/contract.svelte";

	onMount(async () => {
		get_contracts().await;
	});
</script>

<div class="flex h-full">
	{#each $contracts as contract}
		<Contract {contract} />
	{/each}
</div>
