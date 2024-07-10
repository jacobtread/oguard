<script lang="ts">
	import {
		ACTION_TYPE_KEYS,
		ActionTypeKey,
		type Action,
		type ActionDelay,
		type ActionRepeat,
		type ActionRetry,
		type ActionType,
		type ActionTypeConfig
	} from '$lib/api/types';
	import { Collapsible } from 'bits-ui';
	import ActionTypeItem from './ActionTypeItem.svelte';
	import ExecutableConfig from './config/ExecutableConfig.svelte';
	import HttpConfig from './config/HTTPConfig.svelte';
	import ShutdownConfig from './config/ShutdownConfig.svelte';
	import UpsShutdownConfig from './config/UPSShutdownConfig.svelte';
	import { _ } from 'svelte-i18n';
	import ExpandIcon from '~icons/solar/double-alt-arrow-down-bold-duotone';
	import { fly } from 'svelte/transition';

	export let onSubmit: (action: Action) => void;
	export let onCancel: () => void;

	enum State {
		Initial,
		Configure
	}

	let state = State.Initial;
	let selectedActionType: ActionTypeKey = ActionTypeKey.Notification;

	type ActionConfigMap = {
		// Mapping from the type key to the value
		[K in ActionTypeKey]: ActionTypeConfig<K>;
	};

	let actionConfigs: ActionConfigMap = {
		[ActionTypeKey.Notification]: {},
		[ActionTypeKey.Popup]: {},
		[ActionTypeKey.Sleep]: {},
		[ActionTypeKey.Shutdown]: { message: '', timeout: null, force_close_apps: false },
		[ActionTypeKey.USPShutdown]: { delay_minutes: 1 },
		[ActionTypeKey.Executable]: { exe: 'notepad.exe', args: [], timeout: null },
		[ActionTypeKey.HttpRequest]: {
			url: 'https://example.com',
			method: 'GET',
			headers: {},
			body: null,
			timeout: null
		}
	};

	let delay: ActionDelay = {
		duration: null,
		below_capacity: null
	};

	let repeat: ActionRepeat | null = null;
	let retry: ActionRetry | null = null;

	let configScreen: Partial<Record<ActionTypeKey, unknown>> = {
		[ActionTypeKey.Shutdown]: ShutdownConfig,
		[ActionTypeKey.USPShutdown]: UpsShutdownConfig,
		[ActionTypeKey.Executable]: ExecutableConfig,
		[ActionTypeKey.HttpRequest]: HttpConfig
	};

	$: CurrentConfigScreen = configScreen[selectedActionType];

	function handleSubmit() {
		const config: ActionType = {
			type: selectedActionType,
			...actionConfigs[selectedActionType]
		} as unknown as ActionType;

		// Clean up invalid fields
		switch (selectedActionType) {
			case ActionTypeKey.Shutdown: {
				const cfg = config as ActionTypeConfig<ActionTypeKey.Shutdown>;

				cfg.message = cfg.message?.trim() ?? null;

				// Clear empty message
				if (cfg.message !== null && cfg.message.length < 1) {
					cfg.message = null;
				}

				// Clear invalid timeouts
				if (cfg.timeout !== null && cfg.timeout <= 0) {
					cfg.timeout = null;
				}

				break;
			}

			case ActionTypeKey.USPShutdown: {
				const cfg = config as ActionTypeConfig<ActionTypeKey.USPShutdown>;

				// Reset invalid timeouts
				if (cfg.delay_minutes < 0) {
					cfg.delay_minutes = 0;
				}

				break;
			}

			case ActionTypeKey.Executable: {
				const cfg = config as ActionTypeConfig<ActionTypeKey.Executable>;

				// Clear invalid timeouts
				if (cfg.timeout !== null && cfg.timeout <= 0) {
					cfg.timeout = null;
				}

				break;
			}

			case ActionTypeKey.HttpRequest: {
				const cfg = config as ActionTypeConfig<ActionTypeKey.HttpRequest>;

				// Clear empty body
				if (cfg.body !== null && cfg.body.length <= 0) {
					cfg.body = null;
				}

				// Clear invalid timeouts
				if (cfg.timeout !== null && cfg.timeout <= 0) {
					cfg.timeout = null;
				}

				break;
			}

			default:
				break;
		}

		onSubmit({ ty: config, delay, repeat, retry });
	}
</script>

<div class="background" transition:fly={{ duration: 300, y: -10 }}>
	{#if state === State.Initial}
		<div class="dialog" transition:fly={{ duration: 300, x: -10 }}>
			<div class="dialog__header"><h3>Add Action</h3></div>
			<div class="dialog__content">
				<div class="items">
					{#each ACTION_TYPE_KEYS as actionType}
						<ActionTypeItem
							{actionType}
							selected={selectedActionType === actionType}
							onClick={() => (selectedActionType = actionType)}
						/>
					{/each}
				</div>
			</div>
			<div class="dialog__footer">
				<div class="dialog__footer__actions">
					<button class="button" on:click={() => (state = State.Configure)}>Continue</button>
					<div style="flex: auto;"></div>
					<button class="button button--secondary" on:click={onCancel}>Cancel</button>
				</div>
			</div>
		</div>
	{:else if state === State.Configure}
		<div class="dialog" transition:fly={{ duration: 300, x: 10 }}>
			<div class="dialog__header">
				<h3>
					{$_('action.add', {
						values: { action: $_(`actions.${selectedActionType}.label`) }
					})}
				</h3>
			</div>

			<div class="dialog__content">
				{#if CurrentConfigScreen !== undefined}
					<div class="dialog__section">
						<Collapsible.Root>
							<Collapsible.Trigger>
								{$_('action.settings', {
									values: { setting: $_(`actions.${selectedActionType}.label`) }
								})}
								<span data-collapsible-icon> <ExpandIcon /> </span>
							</Collapsible.Trigger>
							<Collapsible.Content>
								<CurrentConfigScreen bind:config={actionConfigs[selectedActionType]} />
							</Collapsible.Content>
						</Collapsible.Root>
					</div>
				{/if}

				<div class="dialog__section">
					<Collapsible.Root>
						<Collapsible.Trigger>
							{$_('action.delay')}
							<span data-collapsible-icon> <ExpandIcon /> </span>
						</Collapsible.Trigger>
						<Collapsible.Content>Delay</Collapsible.Content>
					</Collapsible.Root>
				</div>
				<div class="dialog__section">
					<Collapsible.Root>
						<Collapsible.Trigger>
							{$_('action.repeat')}
							<span data-collapsible-icon> <ExpandIcon /> </span>
						</Collapsible.Trigger>
						<Collapsible.Content>Repeat</Collapsible.Content>
					</Collapsible.Root>
				</div>
				<div class="dialog__section">
					<Collapsible.Root>
						<Collapsible.Trigger>
							{$_('action.retry')}
							<span data-collapsible-icon> <ExpandIcon /> </span>
						</Collapsible.Trigger>
						<Collapsible.Content>Retry</Collapsible.Content>
					</Collapsible.Root>
				</div>
			</div>
			<div class="dialog__footer">
				<div class="dialog__footer__actions">
					<button class="button" on:click={handleSubmit}>Add</button>
					<div style="flex: auto;"></div>
					<button class="button button--secondary" on:click={() => (state = State.Initial)}>
						Back
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.background {
		position: fixed;
		left: 0;
		top: 0;
		width: 100vw;
		height: 100vh;

		display: flex;
		justify-content: center;
		align-items: center;

		padding: 1rem;

		background-color: #f4f6f8;
		background-color: rgba($color: #000000, $alpha: 0.5);
	}

	.dialog {
		position: fixed;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		width: 100%;
		max-width: 40rem;

		background-color: #fff;
		border: $border;
		border-radius: 0.25rem;
	}

	.dialog__header {
		background-color: palette.$gray-200;
		padding: 1rem;
		border-bottom: $border;
		color: palette.$gray-700;
	}

	.dialog__content {
		max-height: 40rem;
		overflow: auto;
	}

	.dialog__section {
		margin: 1rem;

		border: $border;

		[data-collapsible-root] {
		}

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

	.items {
		display: flex;
		flex-flow: column;
		gap: 0.5rem;
		padding: 1rem;
	}

	.dialog__footer {
		display: flex;
		padding: 1rem;

		justify-content: space-between;

		border-top: $border;
	}

	.dialog__footer__actions {
		display: flex;
		flex: auto;
		align-items: center;
		gap: 1rem;
	}
</style>
