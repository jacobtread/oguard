<script lang="ts">
	import {
		createCancelBatteryTestMutation,
		createDeviceStatePollingQuery,
		createStartBatteryTestMutation
	} from '$lib/api/device';
	import Spinner from '$lib/components/Spinner.svelte';
	import { i18nContext } from '$lib/i18n/i18n.svelte';
	import { Container } from '$lib/components';

	const i18n = i18nContext.get();

	const deviceStateQuery = createDeviceStatePollingQuery(1000);
	const startBatteryTestMutation = createStartBatteryTestMutation();
	const cancelBatteryTestMutation = createCancelBatteryTestMutation();
</script>

<svelte:head>
	<title>OGuard | {i18n.f('pages.realtime')}</title>
</svelte:head>

<Container.Wrapper>
	<Container.Root>
		<Container.Header title={i18n.f('pages.realtime')} />
		<Container.Content>
			<Container.Section>
				<div class="fls">
					<div>
						{#if deviceStateQuery.isPending}
							<Spinner />
						{/if}
						{#if deviceStateQuery.error}
							An error has occurred:
							{deviceStateQuery.error.message}
						{/if}
						{#if deviceStateQuery.isSuccess}
							<div class="fl">
								<div class="fl__text">
									<h3 class="fl__name">{i18n.f('battery_self_test.name')}</h3>
									<p class="fl__description">
										{i18n.f('battery_self_test.description')}
									</p>

									<span class="toggling">
										{#if startBatteryTestMutation.isPending}
											<Spinner />
											{i18n.f('battery_self_test.starting')}
										{/if}
									</span>
									<span class="toggling">
										{#if cancelBatteryTestMutation.isPending}
											<Spinner />
											{i18n.f('battery_self_test.cancelling')}
										{/if}
									</span>

									{#if deviceStateQuery.data.battery_self_test}
										<span class="toggling">
											<Spinner />
											{i18n.f('battery_self_test.running')}
										</span>
									{/if}
								</div>

								<div class="self-test">
									{#if deviceStateQuery.data.battery_self_test}
										<button
											class="button"
											onclick={() => cancelBatteryTestMutation.mutate()}
											disabled={cancelBatteryTestMutation.isPending}>
											{i18n.f('battery_self_test.cancel')}
										</button>
									{:else}
										<button
											class="button"
											onclick={() => startBatteryTestMutation.mutate()}
											disabled={startBatteryTestMutation.isPending}>
											{i18n.f('battery_self_test.start')}
										</button>
									{/if}
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
	@use '$styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.self-test {
		flex-shrink: 0;
	}

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
