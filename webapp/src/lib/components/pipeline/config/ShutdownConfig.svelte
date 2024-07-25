<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.Shutdown>;

	const addMessage = () => (config.message = 'Custom message');
	const removeMessage = () => (config.message = null);
	const addTimeout = () => (config.timeout = { secs: 5, nanos: 0 });
	const removeTimeout = () => (config.timeout = null);
</script>

<div class="field">
	<h4>Message</h4>

	<p class="field__description">
		Message to show in the shutdown dialog, will only appear if a timeout is set
	</p>

	{#if config.message !== null}
		<input class="input" type="text" required bind:value={config.message} />
		<button class="button" on:click={removeMessage}>Remove Message</button>
	{:else}
		<button class="button" on:click={addMessage}>Add Message</button>
	{/if}
</div>

<div class="field">
	<h4>Timeout</h4>

	<p class="field__description">Timer shown on the device before the system will shutdown</p>

	{#if config.timeout === null}
		<button class="button" on:click={addTimeout}>Add Timeout</button>
	{:else}
		<label class="field__label">
			Seconds:
			<input class="input" type="number" bind:value={config.timeout.secs} min="0" max="20" />
		</label>

		<button class="button" on:click={removeTimeout}>Remove Timeout</button>
	{/if}
</div>

<div>
	<h4>Force close apps</h4>

	<p class="field__description">Forcefully terminate running apps to shutdown</p>

	<input type="checkbox" bind:checked={config.force_close_apps} />
</div>
