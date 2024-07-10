<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.USPShutdown>;
</script>

<p>UPS Shutdown config</p>

<input type="number" bind:value={config.delay_minutes} />
