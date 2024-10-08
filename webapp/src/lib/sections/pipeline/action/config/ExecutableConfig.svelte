<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';
	import DurationInput from '$lib/components/DurationInput.svelte';

	export let config: ActionTypeConfig<ActionTypeKey.Executable>;

	const addTimeout = () => (config.timeout = { secs: 100, nanos: 0 });
	const removeTimeout = () => (config.timeout = null);

	let arg = config.args.join(' ');

	function updateArgs() {
		config.args = arg.trim().split(' ');
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-expressions
	$: arg, updateArgs();
</script>

<div class="field">
	<h4>Executable</h4>

	<p class="field__description">
		The executable to run, can be just the executable name if the executable is on the PATH
		otherwise use an absolute path
	</p>

	<input class="input" type="text" bind:value={config.exe} required />
</div>

<div class="field">
	<h4>Arguments</h4>

	<p class="field__description">
		Arguments to run the executable with. You can use the <span>&lbrace;OGUARD_EVENT&rbrace;</span> placeholder
		which will be replaced with the event name
	</p>

	<input class="input" type="text" bind:value={arg} required />
</div>

<p>The following command will run:</p>
<code class="command">{config.exe} {arg}</code>

<div class="field">
	<h4>Timeout</h4>

	<p class="field__description">
		If the execution takes longer than a fixed time stop the program and consider it a failed
		execution
	</p>

	{#if config.timeout === null}
		<button class="button" on:click={addTimeout}>Add Timeout</button>
	{:else}
		<DurationInput bind:duration={config.timeout} />
		<button class="button" on:click={removeTimeout}>Remove Timeout</button>
	{/if}
</div>

<style lang="scss">
	.command {
		background-color: #333;
		padding: 1rem;
		color: #fff;
		font-family: monospace;
	}
</style>
