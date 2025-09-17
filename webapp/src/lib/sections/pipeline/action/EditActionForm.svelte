<script lang="ts">
	import type { Action } from '$lib/api/types';
	import ConfigureActionForm from './ConfigureActionForm.svelte';
	import { fade, scale } from 'svelte/transition';
	import { Dialog } from 'bits-ui';
	import { cloneDeep } from 'lodash';
	import { i18nContext } from '$/lib/i18n/i18n.svelte';

	const i18n = i18nContext.get();

	export let open: boolean;
	export let action: Action | null;

	export let onSubmit: (action: Action) => void;
	export let onCancel: () => void;

	// Operate on a copy of the action rather than the action itself
	let editingAction = cloneDeep(action);

	function setEditingAction(action: Action | null) {
		editingAction = cloneDeep(action);
	}

	$: setEditingAction(action);
</script>

<Dialog.Root
	{open}
	onOpenChange={(open) => {
		if (!open) onCancel();
	}}>
	<Dialog.Portal>
		<Dialog.Overlay transition={fade} transitionConfig={{ duration: 300 }} />
		<Dialog.Content transition={scale} transitionConfig={{ duration: 300, start: 0.95 }}>
			<div class="dialog__header"><h3>Edit Action</h3></div>

			{#if editingAction !== null}
				<div class="dialog__subheader">
					<h3>
						{i18n.f('action.configure', {
							values: { action: i18n.f(`actions.${editingAction.ty.type}.label`) }
						})}
					</h3>
				</div>

				<div class="dialog__content">
					<ConfigureActionForm bind:action={editingAction} />
				</div>
				<div class="dialog__footer">
					<div class="dialog__footer__actions">
						<button
							class="button"
							on:click={() => editingAction !== null && onSubmit(editingAction)}>Update</button>
						<div style="flex: auto;"></div>
						<button class="button button--secondary" on:click={onCancel}>Cancel</button>
					</div>
				</div>
			{/if}
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

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
