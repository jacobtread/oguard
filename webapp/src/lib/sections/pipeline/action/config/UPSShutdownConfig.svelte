<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.USPShutdown>;
</script>

<div class="field">
	<h4>Delay</h4>

	<p class="field__description">
		Delay before the UPS will shutdown, ensure this is a sufficient amount of time for the device to
		shutdown and be sure to include a shutdown action before this
	</p>

	<input class="input" type="number" bind:value={config.delay_minutes} />
</div>
