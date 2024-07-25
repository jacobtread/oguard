<script lang="ts">
	import type { DeviceState } from '$/lib/api/types';
	import { HttpMethod, requestJson, requestText } from '$/lib/api/utils';
	import { Container } from '$lib/components';
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { Switch } from 'bits-ui';
	import { t } from 'svelte-i18n';

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

<Container.Wrapper>
	<Container.Root>
		<Container.Header title={$t('pages.configure')} />
		<Container.Content>
			<Container.Section>
				<div class="fls">
					<div>
						{#if $deviceStateQuery.isPending}
							Loading...
						{/if}
						{#if $deviceStateQuery.error}
							An error has occurred:
							{$deviceStateQuery.error.message}
						{/if}
						{#if $deviceStateQuery.isSuccess}
							<div class="fl">
								<div class="fl__text">
									<h3 class="fl__name">{$t('buzzer.name')}</h3>
									<p class="fl__description">
										{$t('buzzer.description')}
									</p>

									<span class="toggling">
										{#if $toggleBuzzerMutation.isPending}
											Toggling buzzer...
										{/if}
									</span>
								</div>

								<div>
									<Switch.Root
										disabled={$toggleBuzzerMutation.isPending}
										checked={$deviceStateQuery.data.buzzer_control}
										onCheckedChange={() => {
											$toggleBuzzerMutation.mutate();
										}}>
										<Switch.Thumb />
									</Switch.Root>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</Container.Section>
		</Container.Content>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.toggling {
		color: palette.$gray-500;
		font-size: 0.8rem;
		margin-top: 0.25rem;
		display: block;
	}

	.fls {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	// Span the columns for the last child when theres an odd number of items
	.fl:nth-child(odd):last-child {
		grid-column: 1/3;
	}

	.fl {
		border: $border;
		background-color: palette.$gray-100;
		padding: 1rem;
		border-radius: 0.125rem;
		display: flex;
		gap: 1rem;
		width: 100%;
		align-items: center;

		&__text {
			flex: auto;
		}

		&__name {
			font-size: 1rem;
			margin-bottom: 0.25rem;
		}

		&__description {
			font-size: 0.9rem;
			color: palette.$gray-700;
		}
	}
</style>
