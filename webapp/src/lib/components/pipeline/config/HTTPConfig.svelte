<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';
	import DurationInput from '$lib/components/DurationInput.svelte';

	export let config: ActionTypeConfig<ActionTypeKey.HttpRequest>;

	const HTTP_METHODS = ['POST', 'PUT', 'PATCH', 'GET', 'DELETE'];

	let headers: [string, string][] = [];

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

	function updateHeaders() {
		const headerMap: Record<string, string> = {};

		for (const [key, value] of headers) {
			const trimmedKey = key.trim();
			const trimmedValue = value.trim();
			if (trimmedKey.length === 0 || trimmedValue.length === 0) continue;

			headerMap[key] = value;
		}

		config.headers = headerMap;
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-expressions
	$: headers, updateHeaders();
</script>

<p>HTTP config</p>

<div>
	<h4>URL</h4>

	<p>URL to send the request to</p>

	<input type="text" bind:value={config.url} required />
</div>

<div>
	<h4>Method</h4>

	<p>HTTP method to use</p>

	<select bind:value={config.method}>
		{#each HTTP_METHODS as method}
			<option value={method}>{method}</option>
		{/each}
	</select>
</div>

<div>
	<h4>Headers</h4>

	<p>Headers to put on the request</p>
	<ul>
		{#each headers as header, index}
			<li>
				<input type="text" bind:value={header[0]} required />
				<input type="text" bind:value={header[1]} required />

				{#if headers.length > 0}
					<button on:click={() => removeHeader(index)}>Remove</button>
				{/if}
			</li>
		{/each}
	</ul>

	<button on:click={addHeader}>Add Header</button>
</div>

<div>
	<h4>Body</h4>

	<p>
		Add a request body to send, will only be used for POST, PUT, PATCH, and DELETE requests supports
		OGuard variables
	</p>

	{#if config.body === null}
		<button on:click={addBody}>Add Body</button>
	{:else}
		<textarea name="" id="" bind:value={config.body.payload}></textarea>
		<input name="" id="" bind:value={config.body.content_type} />
		<button on:click={removeBody}>Remove Body</button>
	{/if}
</div>

<div>
	<h4>Timeout</h4>

	<p>
		If the request/response takes longer than this timeout the request will fail and be considered a
		failed run
	</p>

	{#if config.timeout === null}
		<button on:click={addTimeout}>Add Timeout</button>
	{:else}
		<DurationInput bind:duration={config.timeout} />
		<button on:click={removeTimeout}>Remove Timeout</button>
	{/if}
</div>
