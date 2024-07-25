<script lang="ts">
	import type { DeviceState } from '$/lib/api/types';
	import { HttpMethod, requestJson, requestText } from '$/lib/api/utils';
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { Switch } from 'bits-ui';

	const client = useQueryClient();

	const deviceStateQuery = createQuery<DeviceState>({
		queryKey: ['device-state'],
		queryFn: async () =>
			await requestJson<DeviceState>({
				method: HttpMethod.GET,
				route: '/api/device-state'
			}),

		// Refetch the data every second
		refetchInterval: 1000
	});

	const toggleBuzzerMutation = createMutation({
		mutationFn: async () =>
			await requestText({
				method: HttpMethod.POST,
				route: '/api/toggle-buzzer'
			}),

		onMutate: async () => {
			await client.cancelQueries({ queryKey: ['device-state'] });

			const previousState = client.getQueryData<DeviceState>(['evice-state']);

			if (previousState) {
				client.setQueryData<DeviceState>(['device-state'], {
					...previousState,
					buzzer_control: !previousState.buzzer_control
				});
			}

			return previousState;
		},

		onSettled: () => {
			client.invalidateQueries({ queryKey: ['device-state'] });
		}
	});
</script>

{#if $deviceStateQuery.isPending}
	Loading...
{/if}
{#if $deviceStateQuery.error}
	An error has occurred:
	{$deviceStateQuery.error.message}
{/if}
{#if $deviceStateQuery.isSuccess}
	<b>{$deviceStateQuery.data.buzzer_control ? 'Buzzer Enabled' : 'Buzzer Disabled'}</b>

	<Switch.Root
		disabled={$toggleBuzzerMutation.isPending}
		checked={$deviceStateQuery.data.buzzer_control}
		onCheckedChange={() => {
			$toggleBuzzerMutation.mutate();
		}}>
		<Switch.Thumb />
	</Switch.Root>
{/if}

{#if $toggleBuzzerMutation.isPending}
	Toggling buzzer...
{/if}
