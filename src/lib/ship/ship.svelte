<script>
	import Spaceship from "virtual:icons/game-icons/spaceship";

	export let ship;
</script>

<div class="flex m-3 border-2 border-cyan-400 p-4">
	<div>
		<Spaceship class="text-cyan-400 text-8xl float-left mr-3" />
	</div>
	<div>
		<p class="border-b-2 border-cyan-400 w-fit pb-1">
			{ship.symbol} - {ship.registration.role}
		</p>
		<br />
		<p>System: {ship.nav.systemSymbol}</p>
		<p>Waypoint: {ship.nav.waypointSymbol}</p>
		<p>Status: {ship.nav.status}</p>
		{#if ship.crew.capacity != 0}
			<div>
				<p>Fuel: {ship.fuel.current}/{ship.fuel.capacity}</p>
			</div>
		{/if}
		{#if ship.crew.required != 0}
			<div class="p-1 mt-1 border-2 border-cyan-400">
				<h1
					class="text-center justify-center mx-auto bg-cyan-400 text-black"
				>
					Crew
				</h1>
				<p>
					Crew: {ship.crew.required}/{ship.crew.current}/{ship.crew
						.capacity}
				</p>
				<p>Morale: {ship.crew.morale}</p>
				<p>Wages: {ship.crew.wages}</p>
				<p>Rotation: {ship.crew.rotation}</p>
			</div>
		{/if}
		{#if ship.cargo.capacity != 0}
			<div class="p-1 mt-1 border-2 border-cyan-400">
				<h1
					class="text-center justify-center mx-auto bg-cyan-400 text-black"
				>
					Cargo
				</h1>
				<p>Cargo: {ship.cargo.units}/{ship.cargo.capacity}</p>
				{#each ship.cargo.inventory as cargo}
					<p>{cargo.symbol} - {cargo.units}</p>
				{/each}
			</div>
		{/if}
	</div>
</div>
