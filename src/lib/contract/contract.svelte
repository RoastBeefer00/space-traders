<script>
	export let contract;

	import Contract from "virtual:icons/material-symbols/contract-edit-rounded";
	import Checkbox from "virtual:icons/material-symbols/check-box-outline";
	import Money from "virtual:icons/ri/money-cny-circle-line";
</script>

<div class="m-auto p-1 border-4 border-cyan-400 flex flex-col">
	<div class="flex bg-cyan-400 text-black p-2">
		<div>
			<Contract class="mr-2 my-auto text-6xl" />
		</div>
		<div>
			<h1 class="text-3xl">{contract.type}</h1>
			<p>{contract.id}</p>
			<p>{contract.factionSymbol}</p>
		</div>
	</div>
	<div class="p-2">
		<p>Deadline: {contract.terms.deadline}</p>
		<div class="flex">
			<span>Accepted: </span>
			{#if contract.accepted}
				<Checkbox class="my-auto text-xl" />
			{:else}
				<span>X</span>
			{/if}
			<span class="ml-2"> ({contract.terms.payment.onAccepted}</span>
			<Money class="ml-2 my-auto text-xl" />
			<span>)</span>
		</div>
		<div class="flex">
			<span>Fulfilled: </span>
			{#if contract.fulfilled}
				<Checkbox class="my-auto text-xl" />
			{:else}
				<span>X</span>
			{/if}
			<span class="ml-2"> ({contract.terms.payment.onFulfilled}</span>
			<Money class="ml-2 my-auto text-xl" />
			<span>)</span>
		</div>
		{#if contract.accepted == "False"}
			<p>Deadline to accept: {contract.deadlineToAccept}</p>
		{/if}
	</div>
	{#each contract.terms.deliver as deliver}
		<div class="border-2 border-cyan-400 p-1">
			<p class="bg-cyan-400 text-black p-2">
				{deliver.tradeSymbol} - {deliver.unitsFulfilled}/{deliver.unitsRequired}
			</p>
			<p class="p-2">Deliver to {deliver.destinationSymbol}</p>
		</div>
	{/each}
</div>
