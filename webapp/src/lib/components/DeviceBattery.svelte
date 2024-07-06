<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from './api';

	type DeviceBattery = {
		capacity: number;
		remaining_time: number;
	};

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
	<div>
		{$batteryInfoQuery.data.capacity}
		{$batteryInfoQuery.data.remaining_time}

		Last fetched {$batteryInfoQuery.dataUpdatedAt}
	</div>
{/if}

{#if $batteryInfoQuery.isFetching}
	Background Updating...
{/if}
