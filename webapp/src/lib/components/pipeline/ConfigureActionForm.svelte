<script lang="ts">
	import {
		ActionRetryDelayKey,
		ActionTypeKey,
		getDefaultActionRetryDelay,
		type Action
	} from '$lib/api/types';
	import { _ } from 'svelte-i18n';
	import ExecutableConfig from './config/ExecutableConfig.svelte';
	import HttpConfig from './config/HTTPConfig.svelte';
	import ShutdownConfig from './config/ShutdownConfig.svelte';
	import UpsShutdownConfig from './config/UPSShutdownConfig.svelte';
	import { Collapsible } from 'bits-ui';
	import ActionDelayConfig from './config/ActionDelayConfig.svelte';
	import ActionRepeatConfig from './config/ActionRepeatConfig.svelte';
	import ActionRetryConfig from './config/ActionRetryConfig.svelte';
	import ExpandIcon from '~icons/solar/double-alt-arrow-down-bold-duotone';
	import { slide } from 'svelte/transition';

	export let action: Action;

	$: actionType = action.ty.type;

	const configScreen: Partial<Record<ActionTypeKey, unknown>> = {
		[ActionTypeKey.Shutdown]: ShutdownConfig,
		[ActionTypeKey.USPShutdown]: UpsShutdownConfig,
		[ActionTypeKey.Executable]: ExecutableConfig,
		[ActionTypeKey.HttpRequest]: HttpConfig
	};

	$: CurrentConfigScreen = configScreen[actionType];

	// Adds a default repeat to the action
	const addDelay = () => {
		action.delay = { duration: { secs: 5, nanos: 0 }, below_capacity: null };
	};

	// Removes the repeat from the action
	const removeDelay = () => {
		action.delay = null;
	};

	// Adds a default repeat to the action
	const addRepeat = () => {
		action.repeat = { interval: null, capacity_decrease: null, limit: 1 };
	};

	// Removes the repeat from the action
	const removeRepeat = () => {
		action.repeat = null;
	};

	// Adds a default retry to the action
	const addRetry = () => {
		action.retry = {
			delay: getDefaultActionRetryDelay(ActionRetryDelayKey.Fixed),
			max_attempts: 1
		};
	};

	// Remove the action retry
	const removeRetry = () => {
		action.retry = null;
	};
</script>

{#if CurrentConfigScreen !== undefined}
	<div class="section">
		<Collapsible.Root>
			<Collapsible.Trigger>
				{$_('action.settings', {
					values: { setting: $_(`actions.${actionType}.label`) }
				})}
				<span data-collapsible-icon> <ExpandIcon /> </span>
			</Collapsible.Trigger>
			<Collapsible.Content transition={slide}>
				<div class="section__content">
					<CurrentConfigScreen bind:config={action.ty} />
				</div>
			</Collapsible.Content>
		</Collapsible.Root>
	</div>
{/if}

<div class="section">
	{#if action.delay === null}
		<div class="section__optional">
			<p>This action will run immediately</p>
			<button class="button" style="align-self: flex-start" on:click={addDelay}>Add Delay</button>
		</div>
	{:else}
		<Collapsible.Root>
			<Collapsible.Trigger>
				{$_('action.delay')}
				<div class="section__actions">
					<span data-collapsible-icon> <ExpandIcon /> </span>
					<button class="button" on:click={removeDelay}>Remove</button>
				</div>
			</Collapsible.Trigger>
			<Collapsible.Content transition={slide}>
				<div class="section__content">
					<ActionDelayConfig bind:delay={action.delay} />
				</div>
			</Collapsible.Content>
		</Collapsible.Root>
	{/if}
</div>
<div class="section">
	{#if action.repeat === null}
		<div class="section__optional">
			<p>This action will not automatically repeat</p>
			<button class="button" style="align-self: flex-start" on:click={addRepeat}>Add Repeat</button>
		</div>
	{:else}
		<Collapsible.Root>
			<Collapsible.Trigger>
				{$_('action.repeat')}
				<div class="section__actions">
					<span data-collapsible-icon> <ExpandIcon /> </span>
					<button class="button" on:click={removeRepeat}>Remove</button>
				</div>
			</Collapsible.Trigger>
			<Collapsible.Content transition={slide}>
				<div class="section__content">
					<ActionRepeatConfig bind:repeat={action.repeat} />
				</div>
			</Collapsible.Content>
		</Collapsible.Root>
	{/if}
</div>
<div class="section">
	{#if action.retry === null}
		<div class="section__optional">
			<p>This action will not retry on failure</p>
			<button class="button" style="align-self: flex-start" on:click={addRetry}>Add Retry</button>
		</div>
	{:else}
		<Collapsible.Root>
			<Collapsible.Trigger>
				{$_('action.retry')}
				<div class="section__actions">
					<span data-collapsible-icon> <ExpandIcon /> </span>
					<button class="button" on:click={removeRetry}>Remove</button>
				</div>
			</Collapsible.Trigger>
			<Collapsible.Content transition={slide}>
				<div class="section__content">
					<ActionRetryConfig bind:retry={action.retry} />
				</div>
			</Collapsible.Content>
		</Collapsible.Root>
	{/if}
</div>

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.section {
		margin: 1rem;

		border: $border;

		:global([data-collapsible-trigger]) {
			background-color: palette.$gray-200;
			width: 100%;
			border: none;
			border-bottom: $border;
			text-align: left;
			display: flex;
			justify-content: space-between;
			font-size: 1rem;
			font-weight: bold;
			padding: 0.5rem 0.5rem 0.5rem 1rem;
			align-items: center;
			cursor: pointer;
			color: palette.$gray-700;
		}

		:global([data-collapsible-icon]) {
			font-size: 1.5rem;
			color: palette.$gray-900;
		}
	}

	.section__optional {
		display: flex;
		flex-flow: row;
		justify-content: space-between;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
	}

	.section__actions {
		display: flex;
		align-items: center;
		gap: 0.4rem;
	}

	.section__content {
		display: flex;
		flex-flow: column;
		gap: 0.5rem;
		padding: 1rem;
	}
</style>
