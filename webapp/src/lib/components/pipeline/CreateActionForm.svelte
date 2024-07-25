<script lang="ts">
	import {
		ACTION_TYPE_KEYS,
		ActionTypeKey,
		getDefaultActionType,
		type Action
	} from '$lib/api/types';
	import { _ } from 'svelte-i18n';
	import ActionTypeItem from './ActionTypeItem.svelte';
	import ConfigureActionForm from './ConfigureActionForm.svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { Dialog } from 'bits-ui';
	import { cloneDeep } from 'lodash';

	export let open: boolean;

	export let onSubmit: (action: Action) => void;
	export let onCancel: () => void;

	enum State {
		// Picking action type
		Initial,
		// Configuring action
		Configure
	}

	let state = State.Initial;

	let action: Action = {
		ty: getDefaultActionType(ActionTypeKey.Notification),
		delay: null,
		repeat: null,
		retry: null
	};

	// Handles changing the current action type
	const onChangeActionType = (actionType: ActionTypeKey) => {
		action.ty = getDefaultActionType(actionType);
	};

	const setDefaults = () => {
		state = State.Initial;
		action = {
			ty: getDefaultActionType(ActionTypeKey.Notification),
			delay: null,
			repeat: null,
			retry: null
		};
	};

	// eslint-disable-next-line @typescript-eslint/no-unused-expressions
	$: open, setDefaults();
</script>

<Dialog.Root
	{open}
	onOpenChange={(open) => {
		if (!open) onCancel();
	}}>
	<Dialog.Portal>
		<Dialog.Overlay transition={fade} transitionConfig={{ duration: 300 }} />
		<Dialog.Content transition={scale} transitionConfig={{ duration: 300, start: 0.95 }}>
			<div class="dialog__header"><h3>Add Action</h3></div>
			<div class="transition-outer">
				{#key state}
					<div class="transition-inner" in:fly={{ duration: 100, x: -10 }}>
						{#if state === State.Initial}
							<div class="dialog__content">
								<div class="items">
									{#each ACTION_TYPE_KEYS as actionType}
										<ActionTypeItem
											{actionType}
											selected={action.ty.type === actionType}
											onClick={() => onChangeActionType(actionType)} />
									{/each}
								</div>
							</div>
							<div class="dialog__footer">
								<div class="dialog__footer__actions">
									<button class="button" on:click={() => (state = State.Configure)}
										>Continue</button>
									<div style="flex: auto;"></div>
									<button class="button button--secondary" on:click={onCancel}>Cancel</button>
								</div>
							</div>
						{:else}
							<div class="dialog__subheader">
								<h3>
									{$_('action.configure', {
										values: { action: $_(`actions.${action.ty.type}.label`) }
									})}
								</h3>
							</div>

							<div class="dialog__content">
								<ConfigureActionForm bind:action />
							</div>
							<div class="dialog__footer">
								<div class="dialog__footer__actions">
									<button class="button" on:click={() => onSubmit(cloneDeep(action))}>Add</button>
									<div style="flex: auto;"></div>
									<button class="button button--secondary" on:click={() => (state = State.Initial)}>
										Back
									</button>
								</div>
							</div>
						{/if}
					</div>
				{/key}
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;
	.transition-outer {
		display: grid;
		grid-template: 1fr 1fr;
	}
	.transition-inner {
		grid-row: 1;
		grid-column: 1;
	}
	.dialog__header {
		background-color: palette.$gray-200;
		padding: 1rem;
		border-bottom: $border;
		color: palette.$gray-700;
	}
	.dialog__subheader {
		padding: 1rem;
		border-bottom: $border;
		color: palette.$gray-700;
	}

	.dialog__content {
		max-height: 40rem;
		overflow: auto;
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
