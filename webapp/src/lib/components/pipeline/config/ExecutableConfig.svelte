<script lang="ts">
	import type { ActionTypeConfig, ActionTypeKey } from '$lib/api/types';

	export let config: ActionTypeConfig<ActionTypeKey.Executable>;
</script>

<p>Executable config</p>

<input type="text" bind:value={config.exe} />
