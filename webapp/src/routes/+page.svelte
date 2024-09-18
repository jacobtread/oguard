<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import DeviceBatteryCard from '$/lib/sections/overview/DeviceBatteryCard.svelte';
	import type { DeviceBattery, DeviceState, DeviceBatteryHistory } from '$lib/api/types';
	import DeviceOutputCard from '$lib/sections/overview/DeviceOutputCard.svelte';
	import dayjs from 'dayjs';
	import { AreaChart, ScaleTypes } from '@carbon/charts-svelte';
	import { type AreaChartOptions } from '@carbon/charts';
	import Spinner from '$/lib/components/Spinner.svelte';
	import { Container } from '$/lib/components';
	import { t } from 'svelte-i18n';
	import Alert, { AlertType } from '$/lib/components/Alert.svelte';

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

<svelte:head>
	<title>OGuard | {$t('pages.home')}</title>
</svelte:head>

<div class="grid">
	{#if $batteryInfoQuery.isPending}
		<Spinner />
	{/if}
	{#if $batteryInfoQuery.error}
		<Alert
			type={AlertType.ERROR}
			message={`Failed to load device battery: ${$batteryInfoQuery.error.message}`} />
	{/if}
	{#if $batteryInfoQuery.isSuccess}
		<DeviceBatteryCard
			capacity={$batteryInfoQuery.data.capacity}
			remainingTime={$batteryInfoQuery.data.remaining_time}
			lastUpdated={$batteryInfoQuery.dataUpdatedAt}
			refreshing={$batteryInfoQuery.isFetching} />
	{/if}

	{#if $deviceStateQuery.isPending}
		<Spinner />
	{/if}
	{#if $deviceStateQuery.error}
		<Alert
			type={AlertType.ERROR}
			message={`Failed to load device state: ${$deviceStateQuery.error.message}`} />
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
	<Container.Root>
		{#if $deviceBatteryHistory.isPending}
			<Spinner />
		{/if}
		{#if $deviceBatteryHistory.error}
			<Alert
				type={AlertType.ERROR}
				message={`Failed to load battery history: ${$deviceBatteryHistory.error.message}`} />
		{/if}
		{#if $deviceBatteryHistory.isSuccess}
			<AreaChart
				data={$deviceBatteryHistory.data.map((value) => ({
					date: value.created_at,
					value: value.state.capacity
				}))}
				{options}
				style="padding:2rem;height:400px;" />
		{/if}
	</Container.Root>
</div>

<style lang="scss">
	.grid {
		display: flex;
		gap: 1rem;
		margin: 1rem;
	}
</style>
