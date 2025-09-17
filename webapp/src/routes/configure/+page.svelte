<script lang="ts">
	import { createDeviceStatePollingQuery, createToggleBuzzerMutation } from '$lib/api/device';
	import Spinner from '$lib/components/Spinner.svelte';
	import { i18nContext } from '$lib/i18n/i18n.svelte';
	import { Container } from '$lib/components';
	import { Switch } from 'bits-ui';

	const i18n = i18nContext.get();

	// Query device state, refetching every second
	const deviceStateQuery = createDeviceStatePollingQuery(1000);

	// Mutation to toggle the buzzer state
	const toggleBuzzerMutation = createToggleBuzzerMutation();
</script>

<svelte:head>
	<title>OGuard | {i18n.f('pages.configure')}</title>
</svelte:head>

<Container.Wrapper>
	<Container.Root>
		<Container.Header title={i18n.f('pages.configure')} />
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
									<h3 class="fl__name">{i18n.f('buzzer.name')}</h3>
									<p class="fl__description">
										{i18n.f('buzzer.description')}
									</p>

									<span class="toggling">
										{#if toggleBuzzerMutation.isPending}
											<Spinner />
											Toggling buzzer...
										{/if}
									</span>
								</div>

								<div>
									<Switch.Root
										disabled={toggleBuzzerMutation.isPending}
										checked={deviceStateQuery.data.buzzer_control}
										onCheckedChange={() => {
											toggleBuzzerMutation.mutate();
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
	@use '$styles/palette.scss' as palette;

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
