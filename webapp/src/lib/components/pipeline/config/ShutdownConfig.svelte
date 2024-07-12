<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.Shutdown>;

	const addTimeout = () => (config.timeout = { secs: 5, nanos: 0 });
	const removeTimeout = () => (config.timeout = null);
</script>

<p>Shutdown config</p>

<div>
	<h4>Message</h4>

	<p>Message to show in the shutdown dialog, will only appear if a timeout is set</p>

	<input type="text" bind:value={config.message} />
</div>

<div>
	<h4>Timeout</h4>

	<p>Timer shown on the device before the system will shutdown</p>

	{#if config.timeout === null}
		<button on:click={addTimeout}>Add Timeout</button>
	{:else}
		<label>
			Seconds:
			<input type="number" bind:value={config.timeout.secs} min="0" max="20" />
		</label>

		<button on:click={removeTimeout}>Remove Timeout</button>
	{/if}
</div>

<div>
	<h4>Force close apps</h4>

	<p>Forcefully terminate running apps to shutdown</p>

	<input type="checkbox" bind:checked={config.force_close_apps} />
</div>
