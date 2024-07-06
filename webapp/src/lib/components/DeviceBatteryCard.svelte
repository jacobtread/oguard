<script lang="ts">
	import dayjs from 'dayjs';

	// The current capacity % of the battery
	export let capacity: number;

	// Remaining time in seconds the battery has left
	export let remainingTime: number;

	// When the device battery was last updated
	export let lastUpdated: number;

	const remainingTimeFormatted = dayjs.duration(remainingTime, 'seconds').humanize();
	const lastUpdatedFormatted = dayjs(lastUpdated).format('LT');

	function getBatteryLevelClass(capacity: number) {
		if (capacity < 15) return 'battery-level alert';
		if (capacity < 30) return 'battery-level warn';
		return 'battery-level';
	}
</script>

<div class="card battery-card">
	<div class="battery">
		<div class="battery-level-wrapper">
			<div class={getBatteryLevelClass(capacity)} style="width:{capacity}%;"></div>
		</div>
	</div>

	<p class="battery-capacity"><span class="battery-capacity__value">{capacity}%</span> Capacity</p>

	<p class="battery-remaining">Remaining: {remainingTimeFormatted}</p>

	<p class="battery-last-fetched">Last fetched {lastUpdatedFormatted}</p>
</div>

<style lang="scss">
	.battery-card {
		display: inline-block;

		margin: 1rem;
		min-width: 300px;
	}

	.battery-capacity {
		color: #111;
		font-weight: bold;
		margin-bottom: 0.5em;

		&__value {
			color: #1976d2;
		}
	}

	.battery-remaining {
		color: #222;
		font-style: italic;
		font-size: 0.9rem;
		margin-bottom: 0.5em;
	}

	.battery {
		display: block;
		border: 1px solid #ccc;
		width: 100%;
		height: 3rem;
		border-radius: 0.2rem;
		position: relative;
		padding: 0.1rem;
		box-shadow: 0.25rem 0.25rem 2px rgba(0, 0, 0, 0.1);

		margin-bottom: 0.5rem;
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

	.battery-last-fetched {
		font-size: 0.8rem;
		color: #333;
	}
</style>
