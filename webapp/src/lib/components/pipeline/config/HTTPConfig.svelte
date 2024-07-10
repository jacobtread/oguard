<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.HttpRequest>;
</script>

<p>HTTP config</p>

<input type="text" bind:value={config.url} />
