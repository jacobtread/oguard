<script lang="ts">
	import {
		ACTION_RETRY_DELAY_KEYS,
		ActionRetryDelayKey,
		getDefaultActionRetryDelay,
		type ActionRetry
	} from '$lib/api/types';
	import { Select } from 'bits-ui';
	import DurationInput from '$lib/components/DurationInput.svelte';
	import { _ } from 'svelte-i18n';
	import { fly } from 'svelte/transition';

	export let retry: ActionRetry;

	$: options = ACTION_RETRY_DELAY_KEYS.map((value) => ({
		value,
		label: $_(`action.retry_keys.${value}.label`),
		description: $_(`action.retry_keys.${value}.description`)
	}));

	$: selected = options.find((value) => value.value === retry.delay.type);

	function onChangeType(option: { value: ActionRetryDelayKey } | undefined) {
		if (option === undefined) return;
		const newType = option.value;
		if (retry.delay.type !== newType) {
			retry.delay = getDefaultActionRetryDelay(newType);
		}
	}
</script>

<div class="field">
	<h4>Retry Delay</h4>

	<p class="field__description">Method of delaying the next retry attempt</p>

	<Select.Root items={options} onSelectedChange={onChangeType} {selected}>
		<Select.Trigger>{$_(`action.retry_keys.${retry.delay.type}.label`)}</Select.Trigger>
		<Select.Content
			transition={fly}
			transitionConfig={{ duration: 150, y: -10 }}
			sideOffset={8}
			sameWidth={false}>
			{#each options as option}
				<Select.Item value={option.value} label={option.label}>
					<div class="delay-type">
						<p class="delay-type__label">{option.label}</p>
						<p class="delay-type__description">{option.description}</p>
					</div>
				</Select.Item>
			{/each}
		</Select.Content>
		<Select.Input value={retry.delay.type} />
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

<style lang="scss">
	.delay-type {
		display: flex;
		padding: 0.5rem 1rem;

		display: flex;
		flex-flow: column;
		gap: 0.25rem;

		&__label {
			font-weight: bold;
		}

		&__description {
			font-size: 0.9rem;
		}
	}
</style>
