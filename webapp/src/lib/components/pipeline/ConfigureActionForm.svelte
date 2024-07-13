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
	import ActionDelayConfig from './ActionDelayConfig.svelte';
	import ActionRepeatConfig from './ActionRepeatConfig.svelte';
	import ActionRetryConfig from './ActionRetryConfig.svelte';
	import ExpandIcon from '~icons/solar/double-alt-arrow-down-bold-duotone';

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
			<Collapsible.Content>
				<CurrentConfigScreen bind:config={action.ty} />
			</Collapsible.Content>
		</Collapsible.Root>
	</div>
{/if}

<div class="section">
	<Collapsible.Root>
		<Collapsible.Trigger>
			{$_('action.delay')}
			<span data-collapsible-icon> <ExpandIcon /> </span>
		</Collapsible.Trigger>
		<Collapsible.Content>
			{#if action.delay === null}
				<p>This action will run immediately</p>
				<button on:click={addDelay}>Add Delay</button>
			{:else}
				<ActionDelayConfig bind:delay={action.delay} />
				<button on:click={removeDelay}>Remove Delay</button>
			{/if}
		</Collapsible.Content>
	</Collapsible.Root>
</div>
<div class="section">
	<Collapsible.Root>
		<Collapsible.Trigger>
			{$_('action.repeat')}
			<span data-collapsible-icon> <ExpandIcon /> </span>
		</Collapsible.Trigger>
		<Collapsible.Content>
			{#if action.repeat === null}
				<p>This action will not automatically repeat</p>
				<button on:click={addRepeat}>Add Repeat</button>
			{:else}
				<ActionRepeatConfig bind:repeat={action.repeat} />
				<button on:click={removeRepeat}>Remove Repeating</button>
			{/if}
		</Collapsible.Content>
	</Collapsible.Root>
</div>
<div class="section">
	<Collapsible.Root>
		<Collapsible.Trigger>
			{$_('action.retry')}
			<span data-collapsible-icon> <ExpandIcon /> </span>
		</Collapsible.Trigger>
		<Collapsible.Content>
			{#if action.retry === null}
				<p>This action will not retry on failure</p>
				<button on:click={addRetry}>Add Retry</button>
			{:else}
				<ActionRetryConfig bind:retry={action.retry} />
				<button on:click={removeRetry}>Remove Retry</button>
			{/if}</Collapsible.Content
		>
	</Collapsible.Root>
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
			padding: 0.25rem 1rem;
			align-items: center;
			cursor: pointer;
			color: palette.$gray-700;
		}

		:global([data-collapsible-icon]) {
			font-size: 1.5rem;
			color: palette.$gray-900;
		}
	}
</style>
