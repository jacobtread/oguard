<script lang="ts">
	import SolarBoltCircleBoldDuotone from '~icons/solar/bolt-circle-bold-duotone';
	import SolarRefreshLineDuotone from '~icons/solar/refresh-line-duotone';
	import Container from '$lib/components/container';

	import { t } from 'svelte-i18n';
	import dayjs from 'dayjs';

	// The current capacity % of load on the battery
	export let load: number;

	// Input voltage of the UPS
	export let inputVoltage: number;

	// Output voltage of the UPS
	export let outputVoltage: number;

	// When the device output was last updated
	export let lastUpdated: number;

	// Whether new data is currently being requested
	export let refreshing: boolean;

	// Timestamp in human readable formats
	const lastUpdatedFormatted = dayjs(lastUpdated).format('LT');
</script>

<div class="wrapper">
	<Container.Root>
		<div class="output-header">
			<div class="output">
				<div class="output-level-wrapper">
					<div
						class="output-level"
						class:alert={load > 85}
						class:warn={load <= 85 && load > 70}
						style="width:{load}%;">
					</div>
				</div>
			</div>

			<p class="output-capacity">
				<span class="output-capacity__value">{load}%</span>
				{$t('load')}
			</p>
			<p class="output-remaining">
				<SolarBoltCircleBoldDuotone />
				{$t('input_voltage', { values: { voltage: inputVoltage } })}V
				<SolarBoltCircleBoldDuotone />
				{$t('output_voltage', { values: { voltage: outputVoltage } })}V
			</p>
		</div>

		<div class="card-content">
			<p class="output-last-fetched">
				{$t('last_fetched', { values: { at: lastUpdatedFormatted } })}
			</p>

			{#if refreshing}
				<div class="refresh">
					<SolarRefreshLineDuotone />
					{$t('refreshing')}
				</div>
			{/if}
		</div>
	</Container.Root>
</div>

<style lang="scss">
	.output-header {
		display: block;
		width: 100%;
		background-color: #34495e;
		padding: 1rem;
	}

	.refresh {
		position: absolute;
		right: 0.5rem;
		bottom: 0.5rem;
		font-size: 0.8rem;
		padding: 0.2rem;
		background-color: #ffffff;
		color: #999;
		border-radius: 0.5rem;

		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.wrapper {
		position: relative;
		display: inline-block;

		min-width: 300px;
		width: 50%;
		max-width: 400px;
	}

	.output-capacity {
		color: #fff;
		font-weight: bold;
		margin-top: 0.75em;
		margin-bottom: 0.5em;

		&__value {
			color: #fff;
		}
	}

	.output-remaining {
		font-size: 0.9rem;
		display: flex;

		gap: 0.5rem;
		color: #ccc;
	}

	.output {
		background-color: #293746;
		display: block;
		border: 0.15rem solid #43576d;
		width: 100%;
		height: 3rem;
		border-radius: 0.25rem;
		position: relative;
		padding: 0.2rem;
		box-shadow: 0.25rem 0.25rem 2px rgba(0, 0, 0, 0.1);
	}

	.output-level-wrapper {
		display: block;
		width: 100%;
		height: 100%;
		position: relative;
	}

	.output-level {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 100%;
		border-radius: 0.125rem;

		background: linear-gradient(90deg, #30b455 25%, #3b8650 50%, #30b455 75%);
		background-size: 200% 100%;
		animation: charge 2s infinite ease;

		&.warn {
			background: linear-gradient(90deg, #efaf13 25%, #b48d2a 50%, #efaf13 75%);
			background-size: 200% 100%;
			animation: charge 2s infinite reverse ease;
		}

		&.alert {
			background: linear-gradient(90deg, #e81309 25%, #86201b 50%, #e81309 75%);
			background-size: 200% 100%;
			animation: charge 2s infinite reverse ease;
		}
	}

	@keyframes charge {
		0% {
			background-position: 200% 0;
		}
		50% {
			background-position: 100% 0;
		}
		100% {
			background-position: 0% 0;
		}
	}

	.output-last-fetched {
		font-size: 0.8rem;
		color: #333;
	}
</style>
