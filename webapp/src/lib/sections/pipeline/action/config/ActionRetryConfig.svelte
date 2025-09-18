<script lang="ts">
	import {
		ACTION_RETRY_DELAY_KEYS,
		ActionRetryDelayKey,
		getDefaultActionRetryDelay,
		type ActionRetry
	} from '$lib/api/types';
	import { Select } from 'bits-ui';
	import DurationInput from '$lib/components/DurationInput.svelte';
	import { fly } from 'svelte/transition';
	import { i18nContext } from '$lib/i18n/i18n.svelte';

	const i18n = i18nContext.get();

	interface Props {
		retry: ActionRetry;
	}

	let { retry = $bindable() }: Props = $props();

	const options = $derived(
		ACTION_RETRY_DELAY_KEYS.map((value) => ({
			value,
			label: i18n.f(`action.retry_keys.${value}.label`),
			description: i18n.f(`action.retry_keys.${value}.description`)
		}))
	);

	function onChangeType(value: string) {
		const newType = value as ActionRetryDelayKey;
		if (retry.delay.type !== newType) {
			retry.delay = getDefaultActionRetryDelay(newType);
		}
	}
</script>

<div class="field">
	<h4>Retry Delay</h4>

	<p class="field__description">Method of delaying the next retry attempt</p>

	<Select.Root type="single" items={options} value={retry.delay.type} onValueChange={onChangeType}>
		<Select.Trigger>{i18n.f(`action.retry_keys.${retry.delay.type}.label`)}</Select.Trigger>
		<Select.Portal>
			<Select.Content sideOffset={8}>
				{#snippet child({ open, props, wrapperProps })}
					<div {...wrapperProps}>
						{#if open}
							<div {...props} transition:fly={{ duration: 150, y: -10 }}>
								{#each options as option}
									<Select.Item value={option.value} label={option.label}>
										<div class="delay-type">
											<p class="delay-type__label">{option.label}</p>
											<p class="delay-type__description">{option.description}</p>
										</div>
									</Select.Item>
								{/each}
							</div>
						{/if}
					</div>
				{/snippet}
			</Select.Content>
		</Select.Portal>
	</Select.Root>
</div>

{#if retry.delay.type === ActionRetryDelayKey.Fixed}
	<div class="field">
		<h4>Fixed Duration</h4>

		<p class="field__description">Maximum number of times to retry before failing</p>

		<DurationInput bind:duration={retry.delay.delay} />
	</div>
{:else if retry.delay.type === ActionRetryDelayKey.LinearBackoff}
	<div class="field">
		<h4>Initial Delay</h4>

		<p class="field__description">Starting delay to wait before first retry</p>

		<DurationInput bind:duration={retry.delay.initial} />
	</div>
	<div class="field">
		<h4>Increment</h4>

		<p class="field__description">Amount to increase by after each failed attempt</p>

		<DurationInput bind:duration={retry.delay.increment} />
	</div>
{:else if retry.delay.type === ActionRetryDelayKey.ExponentialBackoff}
	<div class="field">
		<h4>Initial Delay</h4>

		<p class="field__description">Starting delay to wait before first retry</p>

		<DurationInput bind:duration={retry.delay.initial} />
	</div>
	<div class="field">
		<h4>Exponent</h4>

		<p class="field__description">Exponent to multiply the previous delay by</p>

		<input bind:value={retry.delay.exponent} min="2" />
	</div>
{/if}

<div class="field">
	<h4>Max Attempts</h4>

	<p class="field__description">Maximum number of times to retry before failing</p>

	<label class="field__label" for="maxAttempts"> Max Attempts: </label>
	<input id="maxAttempts" class="input" type="number" bind:value={retry.max_attempts} min="1" />
</div>

<style>
	.delay-type {
		display: flex;
		padding: 0.5rem 1rem;

		display: flex;
		flex-flow: column;
		gap: 0.25rem;
	}

	.delay-type__label {
		font-weight: bold;
	}

	.delay-type__description {
		font-size: 0.9rem;
	}
</style>
