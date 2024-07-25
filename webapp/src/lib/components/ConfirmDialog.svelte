<script lang="ts">
	import { Dialog } from 'bits-ui';
	import { fade, scale } from 'svelte/transition';

	export let title: string;
	export let content: string;

	export let open: boolean;

	export let onConfirm: () => void;
	export let onCancel: () => void;
</script>

<Dialog.Root
	{open}
	onOpenChange={(open) => {
		if (!open) onCancel();
	}}>
	<Dialog.Portal>
		<Dialog.Overlay transition={fade} transitionConfig={{ duration: 300 }} />
		<Dialog.Content transition={scale} transitionConfig={{ duration: 300, start: 0.95 }}>
			<div class="dialog__header"><h3>{title}</h3></div>

			<div class="dialog__content">
				<p>{content}</p>
			</div>
			<div class="dialog__footer">
				<div class="dialog__footer__actions">
					<button class="button" on:click={onConfirm}>Confirm</button>
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

	.dialog__content {
		max-height: 40rem;
		overflow: auto;
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
