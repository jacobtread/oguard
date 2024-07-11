<script lang="ts">
	import type { ActionDelay } from '$lib/api/types';
	import DurationInput from '../DurationInput.svelte';

	export let delay: ActionDelay;

	const addFixedDelay = () => (delay.duration = { secs: 100, nanos: 0 });
	const addPercentageDelay = () => (delay.below_capacity = 50);
	const removeFixedDelay = () => (delay.duration = null);
	const removePercentageDelay = () => (delay.below_capacity = null);
</script>

<p>If you add multiple delays the action will execute when the first delay completes</p>

<div>
	<h4>Fixed Delay</h4>

	<p>Add a fixed duration delay, action will run after this duration elapses</p>

	{#if delay.duration === null}
		<button on:click={addFixedDelay}>Add Fixed Delay</button>
	{:else}
		<DurationInput bind:duration={delay.duration} />
		<button on:click={removeFixedDelay}>Remove Fixed Delay</button>
	{/if}
</div>

<div>
	<h4>Below Capacity %</h4>

	<p>Run the action when the UPS battery goes below a specific percentage</p>

	{#if delay.below_capacity === null}
		<button on:click={addPercentageDelay}>Add Capacity Delay</button>
	{:else}
		<label>
			Capacity:
			<input type="number" bind:value={delay.below_capacity} min="0" max="100" />
		</label>
		<button on:click={removePercentageDelay}>Remove Capacity Delay</button>
	{/if}
</div>
