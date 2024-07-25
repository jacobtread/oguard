<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import DeviceBatteryCard from '$lib/components/DeviceBatteryCard.svelte';
	import type { DeviceBattery, DeviceState, DeviceBatteryHistory } from '$lib/api/types';
	import DeviceOutputCard from '$lib/components/DeviceOutputCard.svelte';
	import dayjs from 'dayjs';
	import { AreaChart, ScaleTypes } from '@carbon/charts-svelte';
	import { type AreaChartOptions } from '@carbon/charts';

	const batteryInfoQuery = createQuery<DeviceBattery>({
		queryKey: ['battery-info'],
		queryFn: async () =>
			await requestJson<DeviceBattery>({
				method: HttpMethod.GET,
				route: '/api/battery-state'
			}),

		// Refetch the data every 3 seconds
		refetchInterval: 3000
	});

	const deviceStateQuery = createQuery<DeviceState>({
		queryKey: ['device-state'],
		queryFn: async () =>
			await requestJson<DeviceState>({
				method: HttpMethod.GET,
				route: '/api/device-state'
			}),

		// Refetch the data every 3 seconds
		refetchInterval: 3000
	});

	const deviceBatteryHistory = createQuery<DeviceBatteryHistory[]>({
		queryKey: ['device-battery-history'],
		queryFn: async () => {
			const currentDate = dayjs.utc();
			const startOfDay = currentDate.startOf('day');
			const endOfDay = currentDate.endOf('day');

			const query = new URLSearchParams();
			query.set('start', startOfDay.toISOString());
			query.set('end', endOfDay.toISOString());
			return await requestJson<DeviceBatteryHistory[]>({
				method: HttpMethod.GET,
				route: '/api/history/battery-state?' + query.toString()
			});
		},

		// Refetch the data every minute
		refetchInterval: 1000 * 60
	});

	const options: AreaChartOptions = {
		title: 'Battery Capacity (Today)',
		canvasZoom: { enabled: true },
		locale: {
			date(value) {
				return dayjs(value).format('L LT');
			}
		},
		zoomBar: {
			top: { enabled: true }
		},
		axes: {
			bottom: {
				title: 'Date',
				mapsTo: 'date',
				scaleType: ScaleTypes.TIME
			},
			left: {
				mapsTo: 'value',
				title: 'Battery Capacity',
				scaleType: ScaleTypes.LINEAR,
				percentage: true
			}
		},

		curve: 'curveMonotoneX',
		height: '400px'
	};
</script>

<div class="grid">
	{#if $batteryInfoQuery.isPending}
		Loading...
	{/if}
	{#if $batteryInfoQuery.error}
		An error has occurred:
		{$batteryInfoQuery.error.message}
	{/if}
	{#if $batteryInfoQuery.isSuccess}
		<DeviceBatteryCard
			capacity={$batteryInfoQuery.data.capacity}
			remainingTime={$batteryInfoQuery.data.remaining_time}
			lastUpdated={$batteryInfoQuery.dataUpdatedAt}
			refreshing={$batteryInfoQuery.isFetching} />
	{/if}

	{#if $deviceStateQuery.isPending}
		Loading...
	{/if}
	{#if $deviceStateQuery.error}
		An error has occurred:
		{$deviceStateQuery.error.message}
	{/if}
	{#if $deviceStateQuery.isSuccess}
		<DeviceOutputCard
			load={$deviceStateQuery.data.output_load_percent}
			inputVoltage={$deviceStateQuery.data.input_voltage}
			outputVoltage={$deviceStateQuery.data.output_voltage}
			lastUpdated={$deviceStateQuery.dataUpdatedAt}
			refreshing={$deviceStateQuery.isFetching} />
	{/if}
</div>
<div class="grid">
	{#if $deviceBatteryHistory.isPending}
		Loading...
	{/if}
	{#if $deviceBatteryHistory.error}
		An error has occurred:
		{$deviceBatteryHistory.error.message}
	{/if}
	{#if $deviceBatteryHistory.isSuccess}
		<div class="card graph-card">
			<AreaChart
				data={$deviceBatteryHistory.data.map((value) => ({
					date: value.created_at,
					value: value.state.capacity
				}))}
				{options}
				style="padding:2rem;height:400px;" />
		</div>
	{/if}
</div>

<style lang="scss">
	.grid {
		display: flex;
		gap: 1rem;
		margin: 1rem;
	}

	.graph-card {
		flex: auto;
	}
</style>
