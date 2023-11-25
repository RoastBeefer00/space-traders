<script>
	import { onMount } from "svelte";
	import { get_ships } from "$lib/helpers/helpers.js";
	import { ships } from "$lib/stores.js";
	import Ship from "../../lib/ship/ship.svelte";

	onMount(async () => {
		get_ships().await;
	});
</script>

<div>
	<div class="flex h-screen">
		<div class="m-auto text-xl">
			<div class="flex flex-col">
				{#each $ships as ship}
					<Ship {ship} />
				{/each}
			</div>
		</div>
	</div>
</div>
