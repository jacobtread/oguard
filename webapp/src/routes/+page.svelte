<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import DeviceBatteryCard from '$lib/components/DeviceBatteryCard.svelte';
	import type { DeviceBattery, DeviceState } from '$lib/api/types';
	import DeviceOutputCard from '$lib/components/DeviceOutputCard.svelte';
	const batteryInfoQuery = createQuery<DeviceBattery>({
		queryKey: ['battery-info'],
		queryFn: async () =>
			await requestJson<DeviceBattery>({
				method: HttpMethod.GET,
				route: '/api/battery-state'
			}),

		// Refetch the data every second
		refetchInterval: 3000
	});

	const deviceStateQuery = createQuery<DeviceState>({
		queryKey: ['device-state'],
		queryFn: async () =>
			await requestJson<DeviceState>({
				method: HttpMethod.GET,
				route: '/api/device-state'
			}),

		// Refetch the data every second
		refetchInterval: 3000
	});
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
			refreshing={$batteryInfoQuery.isFetching}
		/>
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
			refreshing={$deviceStateQuery.isFetching}
		/>
	{/if}
</div>

<style lang="scss">
	.grid {
		display: flex;
		gap: 1rem;
		margin: 1rem;
	}
</style>
