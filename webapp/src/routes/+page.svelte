<script lang="ts">
	import DeviceBatteryCard from '$/lib/sections/overview/DeviceBatteryCard.svelte';
	import DeviceOutputCard from '$lib/sections/overview/DeviceOutputCard.svelte';
	import dayjs from 'dayjs';

	import Spinner from '$/lib/components/Spinner.svelte';
	import { Container } from '$/lib/components';
	import { t } from 'svelte-i18n';
	import Alert, { AlertType } from '$/lib/components/Alert.svelte';
	import { createBatteryInfoPollingQuery, createDeviceStatePollingQuery } from '$/lib/api/device';
	import { readable } from 'svelte/store';
	import { createDeviceBatteryHistoryQuery } from '$/lib/api/history';
	import BatteryCapacityChart from '$/lib/components/charts/BatteryCapacityChart.svelte';

	const batteryInfoQuery = createBatteryInfoPollingQuery(3000);
	const deviceStateQuery = createDeviceStatePollingQuery(3000);

	// Get the current date data
	const currentDate = dayjs.utc();
	const startOfDay = readable(currentDate.startOf('day').toDate());
	const endOfDay = readable(currentDate.endOf('day').toDate());

	// Device battery history, refreshing data every minute
	const deviceBatteryHistory = createDeviceBatteryHistoryQuery(startOfDay, endOfDay, 1000 * 60);
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
			<BatteryCapacityChart history={$deviceBatteryHistory.data} />
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
