<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import Ship from "../../lib/ship/ship.svelte";

	import Spaceship from "virtual:icons/game-icons/spaceship";

	let ships = [];
	async function get_ships() {
		ships = await invoke("get_user_ships", {});
	}

	onMount(async () => {
		get_ships().await;
	});
</script>

<div>
	<div class="flex h-screen">
		<div class="m-auto text-xl">
			<div class="flex flex-col">
				{#each ships as ship}
					<Ship {ship} />
				{/each}
			</div>
		</div>
	</div>
</div>
