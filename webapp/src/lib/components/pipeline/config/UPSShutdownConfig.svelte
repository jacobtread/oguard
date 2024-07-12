<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.USPShutdown>;
</script>

<p>UPS Shutdown config</p>

<div>
	<h4>Delay</h4>

	<p>
		Delay before the UPS will shutdown, ensure this is a sufficient amount of time for the device to
		shutdown and be sure to include a shutdown action before this
	</p>

	<input type="number" bind:value={config.delay_minutes} />
</div>
