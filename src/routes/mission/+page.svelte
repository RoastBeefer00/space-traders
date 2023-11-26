<script>
	import { get_waypoints } from "$lib/helpers/helpers.js";
	import { waypoints } from "$lib/stores.js";
	import Page from "../ships/+page.svelte";

	let waypoint_type;
</script>

<div class="p-4">
	<select
		bind:value={waypoint_type}
		class="bg-cyan-400 text-black placeholder-cyan-400"
	>
		<option selected>Choose a waypoint type</option>
		<option value="PLANET">PLANET</option>
		<option value="GAS_GIANT">GAS_GIANT</option>
		<option value="Moon">MOON</option>
		<option value="ORBITAL_STATION">ORBITAL_STATION</option>
		<option value="JUMP_GATE">JUMP_GATE</option>
		<option value="ASTEROID_FIELD">ASTEROID_FIELD</option>
		<option value="ASTEROID">ASTEROID</option>
		<option value="ENGINEERED_ASTEROID">ENGINEERED_ASTEROID</option>
		<option value="ASTEROID_BASE">ASTEROID_BASE</option>
		<option value="NEBULA">NEBULA</option>
		<option value="DEBRIS_FIELD">DEBRIS_FIELD</option>
		<option value="GRAVITY_WELL">GRAVITY_WELL</option>
		<option value="ARTIFICIAL_GRAVITY_WELL">ARTIFICIAL_GRAVITY_WELL</option>
		<option value="FUEL_STATION">FUEL_STATION</option>
	</select>
	<button
		on:click={() => get_waypoints("X1-SU26", waypoint_type, null)}
		class="bg-cyan-400 text-black p-2">Find Waypoints</button
	>
	{#each $waypoints as waypoint}
		<p>{waypoint.symbol}</p>
		<p>{waypoint.type}</p>
		{#each waypoint.traits as trait}
			<p>{trait.symbol}</p>
			<p>{trait.description}</p>
		{/each}
	{/each}
</div>
