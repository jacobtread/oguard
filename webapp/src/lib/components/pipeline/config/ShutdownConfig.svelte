<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.Shutdown>;
</script>

<p>Shutdown config</p>

<input type="text" bind:value={config.message} />
