<script lang="ts">
	import DeviceBatteryCard from '$/lib/sections/overview/DeviceBatteryCard.svelte';
	import DeviceOutputCard from '$lib/sections/overview/DeviceOutputCard.svelte';
	import dayjs from 'dayjs';

	import Spinner from '$/lib/components/Spinner.svelte';
	import { Container } from '$/lib/components';
	import { t } from 'svelte-i18n';
	import Alert, { AlertType } from '$/lib/components/Alert.svelte';
	import { createBatteryInfoPollingQuery, createDeviceStatePollingQuery } from '$/lib/api/device';
	import { createDeviceBatteryHistoryQuery } from '$/lib/api/history';
	import BatteryCapacityChart from '$/lib/components/charts/BatteryCapacityChart.svelte';

	const batteryInfoQuery = createBatteryInfoPollingQuery(3000);
	const deviceStateQuery = createDeviceStatePollingQuery(3000);

	// Get the current date data
	const currentDate = dayjs.utc();
	const startOfDay = currentDate.startOf('day').toDate();
	const endOfDay = currentDate.endOf('day').toDate();

	// Device battery history, refreshing data every minute
	const deviceBatteryHistory = createDeviceBatteryHistoryQuery(
		() => startOfDay,
		() => endOfDay,
		1000 * 60
	);
</script>

<svelte:head>
	<title>OGuard | {$t('pages.home')}</title>
</svelte:head>

<div class="grid">
	<!-- Battery Info -->
	{#if batteryInfoQuery.isPending}
		<Spinner />
	{:else if batteryInfoQuery.error}
		<Alert
			type={AlertType.ERROR}
			message={`Failed to load device battery: ${batteryInfoQuery.error.message}`} />
	{:else if batteryInfoQuery.isSuccess}
		<DeviceBatteryCard
			capacity={batteryInfoQuery.data.capacity}
			remainingTime={batteryInfoQuery.data.remaining_time}
			lastUpdated={batteryInfoQuery.dataUpdatedAt}
			refreshing={batteryInfoQuery.isFetching} />
	{/if}

	<!-- Device state -->
	{#if deviceStateQuery.isPending}
		<Spinner />
	{:else if deviceStateQuery.error}
		<Alert
			type={AlertType.ERROR}
			message={`Failed to load device state: ${deviceStateQuery.error.message}`} />
	{:else if deviceStateQuery.isSuccess}
		<DeviceOutputCard
			load={deviceStateQuery.data.output_load_percent}
			inputVoltage={deviceStateQuery.data.input_voltage}
			outputVoltage={deviceStateQuery.data.output_voltage}
			lastUpdated={deviceStateQuery.dataUpdatedAt}
			refreshing={deviceStateQuery.isFetching} />
	{/if}
</div>

<div class="grid">
	<Container.Root>
		<!-- Battery History -->
		{#if deviceBatteryHistory.isPending}
			<Spinner />
		{:else if deviceBatteryHistory.error}
			<Alert
				type={AlertType.ERROR}
				message={`Failed to load battery history: ${deviceBatteryHistory.error.message}`} />
		{:else if deviceBatteryHistory.isSuccess}
			<BatteryCapacityChart history={deviceBatteryHistory.data} />
		{/if}
	</Container.Root>
</div>

<style>
	.grid {
		display: flex;
		gap: 1rem;
		margin: 1rem;
	}
</style>
