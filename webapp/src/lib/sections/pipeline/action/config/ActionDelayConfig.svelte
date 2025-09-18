<script lang="ts">
	import type { ActionDelay } from '$lib/api/types';
	import DurationInput from '$lib/components/DurationInput.svelte';

	interface Props {
		delay: ActionDelay;
	}

	let { delay = $bindable() }: Props = $props();

	const addFixedDelay = () => (delay.duration = { secs: 100, nanos: 0 });
	const addPercentageDelay = () => (delay.below_capacity = 50);
	const removeFixedDelay = () => (delay.duration = null);
	const removePercentageDelay = () => (delay.below_capacity = null);
</script>

<p class="field__description">
	If you add multiple delays the action will execute when the first delay completes
</p>

<div class="field">
	<h4>Fixed Delay</h4>

	<p class="field__description">
		Add a fixed duration delay, action will run after this duration elapses
	</p>

	{#if delay.duration === null}
		<button class="button" onclick={addFixedDelay}>Add Fixed Delay</button>
	{:else}
		<DurationInput bind:duration={delay.duration} />
		<button class="button" onclick={removeFixedDelay}>Remove Fixed Delay</button>
	{/if}
</div>

<div class="field">
	<h4>Below Capacity %</h4>

	<p class="field__description">
		Run the action when the UPS battery goes below a specific percentage
	</p>

	{#if delay.below_capacity === null}
		<button class="button" onclick={addPercentageDelay}>Add Capacity Delay</button>
	{:else}
		<label>
			Capacity:
			<input class="input" type="number" bind:value={delay.below_capacity} min="0" max="100" />
		</label>
		<button class="button" onclick={removePercentageDelay}>Remove Capacity Delay</button>
	{/if}
</div>
