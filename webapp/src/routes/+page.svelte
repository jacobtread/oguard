<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import DeviceBatteryCard from '$lib/components/DeviceBatteryCard.svelte';
	import type { DeviceBattery } from '$lib/api/types';

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
</script>

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
	/>
{/if}

{#if $batteryInfoQuery.isFetching}
	...
{/if}
