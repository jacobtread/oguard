<script lang="ts">
	import { EventLevel } from '$lib/api/types';
	import InfoIcon from '~icons/solar/info-circle-bold-duotone';
	import WarningIcon from '~icons/solar/danger-triangle-bold-duotone';
	import ErrorIcon from '~icons/solar/bug-bold-duotone';
	import SuccessIcon from '~icons/solar/check-circle-bold-duotone';

	export let level: EventLevel;
</script>

{#if level === EventLevel.Info}
	<span class="level level--info">
		<InfoIcon />
	</span>
{:else if level === EventLevel.Success}
	<span class="level level--success">
		<SuccessIcon />
	</span>
{:else if level === EventLevel.Warning}
	<span class="level level--warning">
		<WarningIcon />
	</span>
{:else if level === EventLevel.Severe}
	<span class="level level--severe">
		<ErrorIcon />
	</span>
{/if}

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.level {
		font-size: 1.25rem;
		line-height: 1;

		&--info {
			color: #34495e;
		}

		&--success {
			color: #30b455;
		}

		&--warning {
			color: #efaf13;
		}

		&--severe {
			color: #aa1109;
		}
	}
</style>
