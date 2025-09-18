<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';
	import DurationInput from '$lib/components/DurationInput.svelte';
	import { watch } from 'runed';

	interface Props {
		config: ActionTypeConfig<ActionTypeKey.HttpRequest>;
	}

	let { config = $bindable() }: Props = $props();

	const HTTP_METHODS = ['POST', 'PUT', 'PATCH', 'GET', 'DELETE'];

	let headers: [string, string][] = $state([]);

	const addHeader = () => {
		headers.push(['', '']);
		headers = headers;
	};

	const removeHeader = (index: number) => {
		headers.splice(index, 1);
		headers = headers;
	};

	const addBody = () => (config.body = { content_type: 'text/plain', payload: '' });
	const removeBody = () => (config.body = null);

	const addTimeout = () => (config.timeout = { secs: 100, nanos: 0 });
	const removeTimeout = () => (config.timeout = null);

	// TODO: Check this synchronizes properly
	watch(
		() => ({ headers }),
		({ headers }) => {
			const headerMap: Record<string, string> = {};

			for (const [key, value] of headers) {
				const trimmedKey = key.trim();
				const trimmedValue = value.trim();
				if (trimmedKey.length === 0 || trimmedValue.length === 0) continue;

				headerMap[key] = value;
			}

			config.headers = headerMap;
		}
	);
</script>

<div class="field">
	<h4>URL</h4>

	<p class="field__description">URL to send the request to</p>

	<input class="input" type="url" bind:value={config.url} required />
</div>

<div class="field">
	<h4>Method</h4>

	<p class="field__description">HTTP method to use</p>

	<select class="input" bind:value={config.method}>
		{#each HTTP_METHODS as method}
			<option value={method}>{method}</option>
		{/each}
	</select>
</div>

<div class="field">
	<h4>Headers</h4>

	<p class="field__description">Headers to put on the request</p>
	<ul>
		{#each headers as header, index}
			<li>
				<input class="input" type="text" bind:value={header[0]} required />
				<input class="input" type="text" bind:value={header[1]} required />

				{#if headers.length > 0}
					<button class="button" onclick={() => removeHeader(index)}>Remove</button>
				{/if}
			</li>
		{/each}
	</ul>

	<button class="button" onclick={addHeader}>Add Header</button>
</div>

<div class="field">
	<h4>Body</h4>

	<p class="field__description">
		Add a request body to send, will only be used for POST, PUT, PATCH, and DELETE requests supports
		OGuard variables
	</p>

	{#if config.body === null}
		<button class="button" onclick={addBody}>Add Body</button>
	{:else}
		<textarea class="input" name="" id="" bind:value={config.body.payload}></textarea>
		<input class="input" name="" id="" bind:value={config.body.content_type} />
		<button class="button" onclick={removeBody}>Remove Body</button>
	{/if}
</div>

<div class="field">
	<h4>Timeout</h4>

	<p class="field__description">
		If the request/response takes longer than this timeout the request will fail and be considered a
		failed run
	</p>

	{#if config.timeout === null}
		<button class="button" onclick={addTimeout}>Add Timeout</button>
	{:else}
		<DurationInput bind:duration={config.timeout} />
		<button class="button" onclick={removeTimeout}>Remove Timeout</button>
	{/if}
</div>
