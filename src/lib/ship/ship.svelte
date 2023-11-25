<script>
	import Spaceship from "virtual:icons/game-icons/spaceship";
	import Crew from "virtual:icons/material-symbols/group-rounded";
	import Smile from "virtual:icons/bi/emoji-smile-fill";
	import Money from "virtual:icons/ri/money-cny-circle-line";
	import Rotate from "virtual:icons/lucide/rotate-ccw";
	import LuggageCart from "virtual:icons/uil/luggage-cart";
	import Fuel from "virtual:icons/material-symbols/local-gas-station-sharp";
	import Rocket from "virtual:icons/material-symbols/rocket-launch-rounded";
	import System from "virtual:icons/lucide/waypoints";
	import Waypoint from "virtual:icons/material-symbols/flag";

	export let ship;
</script>

<div class="flex flex-col border-4 border-cyan-400 p-4 m-2">
	<div class="flex p-3 bg-cyan-400 text-black mb-3">
		<div>
			<Spaceship class="text-8xl float-left mr-3" />
		</div>
		<div>
			<p class="w-fit pb-1 text-3xl">
				{ship.symbol}
				<br />
				<br />
				{ship.registration.role}
			</p>
		</div>
	</div>
	<div>
		<div class="grid grid-cols-2 p-1">
			<p class="flex">
				<System class="my-auto text-xl" />: {ship.nav.systemSymbol}
			</p>
			<p class="flex">
				<Waypoint class="my-auto text-xl" />: {ship.nav.waypointSymbol}
			</p>
			<p class="flex">
				<Rocket class="my-auto text-xl" />: {ship.nav.status}
			</p>
			{#if ship.fuel.capacity != 0}
				<div>
					<p class="flex">
						<Fuel class="my-auto text-xl" />: {ship.fuel
							.current}/{ship.fuel.capacity}
					</p>
				</div>
			{/if}
		</div>
		{#if ship.crew.required != 0}
			<div class="p-1 mt-1 border-2 border-cyan-400">
				<h1
					class="text-center justify-center mx-auto bg-cyan-400 text-black"
				>
					Crew
				</h1>
				<div class="grid grid-cols-2 p-1">
					<p class="flex">
						<Crew class="my-auto text-xl" />: {ship.crew
							.required}/{ship.crew.current}/{ship.crew.capacity}
					</p>
					<p class="flex">
						<Smile class="my-auto text-xl" />: {ship.crew.morale}
					</p>
					<p class="flex">
						<Money class="my-auto text-xl" />: {ship.crew.wages}
					</p>
					<p class="flex">
						<Rotate class="my-auto text-xl" />: {ship.crew.rotation}
					</p>
				</div>
			</div>
		{/if}
		{#if ship.cargo.capacity != 0}
			<div class="p-1 mt-1 border-2 border-cyan-400">
				<h1
					class="text-center justify-center mx-auto bg-cyan-400 text-black"
				>
					Cargo
				</h1>
				<p class="flex">
					<LuggageCart class="my-auto text-2xl" />: {ship.cargo
						.units}/{ship.cargo.capacity}
				</p>
				{#each ship.cargo.inventory as cargo}
					<p>{cargo.symbol} - {cargo.units}</p>
				{/each}
			</div>
		{/if}
	</div>
</div>
