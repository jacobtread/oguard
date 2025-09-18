<script lang="ts">
	import SolarRefreshLineDuotone from '~icons/solar/refresh-line-duotone';
	import Container from '$lib/components/container';

	import dayjs from 'dayjs';
	import { i18nContext } from '$lib/i18n/i18n.svelte';

	interface Props {
		// The current capacity % of the battery
		capacity: number;

		// Remaining time in seconds the battery has left
		remainingTime: number;

		// When the device battery was last updated
		lastUpdated: number;

		// Whether new data is currently being requested
		refreshing: boolean;
	}

	const { capacity, remainingTime, lastUpdated, refreshing }: Props = $props();

	const i18n = i18nContext.get();

	// Timestamp in human readable formats
	const remainingTimeFormatted = $derived(dayjs.duration(remainingTime, 'seconds').humanize());
	const lastUpdatedFormatted = $derived(dayjs(lastUpdated).format('LT'));
</script>

<div class="wrapper">
	<Container.Root>
		<div class="battery-header">
			<div class="battery">
				<div class="battery-level-wrapper">
					<div
						class="battery-level"
						class:alert={capacity < 15}
						class:warn={capacity >= 15 && capacity < 30}
						style="width:{capacity}%;">
					</div>
				</div>
			</div>

			<p class="battery-capacity">
				<span class="battery-capacity__value">{capacity}%</span>
				{i18n.f('capacity')}
			</p>

			<p class="battery-remaining">
				{i18n.f('remaining', { values: { duration: remainingTimeFormatted } })}
			</p>
		</div>

		<div class="card-content">
			<p class="battery-last-fetched">
				{i18n.f('last_fetched', { values: { at: lastUpdatedFormatted } })}
			</p>

			{#if refreshing}
				<div class="refresh">
					<SolarRefreshLineDuotone />
					{i18n.f('refreshing')}
				</div>
			{/if}
		</div>
	</Container.Root>
</div>

<style>
	.battery-header {
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

	.battery-capacity {
		color: #fff;
		font-weight: bold;
		margin-top: 0.75em;
		margin-bottom: 0.5em;
	}

	.battery-capacity__value {
		color: #fff;
	}

	.battery-remaining {
		font-size: 0.9rem;
		gap: 0.5rem;
		color: #ccc;
	}

	.battery {
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

	.battery-level-wrapper {
		display: block;
		width: 100%;
		height: 100%;
		position: relative;
	}

	.battery-level {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 100%;
		border-radius: 0.125rem;

		background: linear-gradient(90deg, #30b455 25%, #3b8650 50%, #30b455 75%);
		background-size: 200% 100%;
		animation: charge 2s infinite ease;
	}

	.battery-level.warn {
		background: linear-gradient(90deg, #efaf13 25%, #b48d2a 50%, #efaf13 75%);
		background-size: 200% 100%;
		animation: charge 2s infinite reverse ease;
	}

	.battery-level.alert {
		background: linear-gradient(90deg, #e81309 25%, #86201b 50%, #e81309 75%);
		background-size: 200% 100%;
		animation: charge 2s infinite reverse ease;
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

	.battery-last-fetched {
		font-size: 0.8rem;
		color: #333;
	}
</style>
