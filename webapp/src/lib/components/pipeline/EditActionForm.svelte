<script lang="ts">
	import type { Action } from '$lib/api/types';
	import { _ } from 'svelte-i18n';
	import ConfigureActionForm from './ConfigureActionForm.svelte';
	import { fly } from 'svelte/transition';
	import { Dialog } from 'bits-ui';
	import { cloneDeep } from 'lodash';

	export let action: Action;

	export let onSubmit: (action: Action) => void;
	export let onCancel: () => void;

	// Operate on a copy of the action rather than the action itself
	let editingAction = cloneDeep(action);

	function setEditingAction(action: Action) {
		editingAction = cloneDeep(action);
	}

	$: setEditingAction(action);
</script>

<Dialog.Root
	open
	onOpenChange={(open) => {
		if (!open) onCancel();
	}}>
	<Dialog.Portal>
		<Dialog.Overlay transition={fly} transitionConfig={{ duration: 300, y: -10 }} />
		<Dialog.Content transition={fly} transitionConfig={{ duration: 300, y: -10 }}>
			<div class="dialog__header"><h3>Edit Action</h3></div>

			<div class="dialog__subheader">
				<h3>
					{$_('action.configure', {
						values: { action: $_(`actions.${action.ty.type}.label`) }
					})}
				</h3>
			</div>

			<div class="dialog__content">
				<ConfigureActionForm bind:action={editingAction} />
			</div>
			<div class="dialog__footer">
				<div class="dialog__footer__actions">
					<button class="button" on:click={() => onSubmit(editingAction)}>Update</button>
					<div style="flex: auto;"></div>
					<button class="button button--secondary" on:click={onCancel}>Cancel</button>
				</div>
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
