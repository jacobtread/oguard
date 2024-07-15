<script lang="ts">
	import type { ActionRepeat } from '$lib/api/types';
	import DurationInput from '../../DurationInput.svelte';

	export let repeat: ActionRepeat;

	const addFixedInterval = () => (repeat.interval = { secs: 100, nanos: 0 });
	const removeFixedInterval = () => (repeat.interval = null);

	const addCapacityDecrease = () => (repeat.capacity_decrease = 10);
	const removeCapacityDecrease = () => (repeat.capacity_decrease = null);

	const addLimit = () => (repeat.limit = 1);
	const removeLimit = () => (repeat.limit = null);
</script>

<p class="field__description">
	The initial execution of the action will happen in the defined order, any repeated executions will
	not maintain order and will run when one of their conditions is met
</p>

<p class="field__description">
	Providing multiple repeat conditions will repeat the action when any of the conditions are met
</p>

<div class="field">
	<h4>Fixed Interval</h4>

	<p class="field__description">Repeat the action at a fixed timing interval</p>

	{#if repeat.interval === null}
		<button class="button" on:click={addFixedInterval}>Add Fixed Interval</button>
	{:else}
		<DurationInput bind:duration={repeat.interval} />
		<button class="button" on:click={removeFixedInterval}>Remove Fixed Interval</button>
	{/if}
</div>

<div class="field">
	<h4>Capacity Decrease %</h4>

	<p class="field__description">
		Run the action whenever the UPS battery goes down by a specific percentage
	</p>

	{#if repeat.capacity_decrease === null}
		<button class="button" on:click={addCapacityDecrease}>Add Capacity Decrease</button>
	{:else}
		<label class="field__label">
			Capacity:
			<input class="input" type="number" bind:value={repeat.capacity_decrease} min="0" max="100" />
		</label>
		<button class="button" on:click={removeCapacityDecrease}>Remove Capacity Decrease</button>
	{/if}
</div>

<div class="field">
	<h4>Limit</h4>

	<p class="field__description">
		Set a limit on how many times this action will repeat, an action will repeat indefinitely
		without a limit
	</p>

	{#if repeat.limit === null}
		<button class="button" on:click={addLimit}>Add Limit</button>
	{:else}
		<label>
			Limit:
			<input class="input" type="number" bind:value={repeat.limit} min="0" />
		</label>
		<button class="button" on:click={removeLimit}>Remove Limit</button>
	{/if}
</div>
