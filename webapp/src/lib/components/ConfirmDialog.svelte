<script lang="ts">
	import { Dialog } from 'bits-ui';
	import { fade, scale } from 'svelte/transition';

	type Props = {
		title: string;
		content: string;
		open: boolean;
		onConfirm: VoidFunction;
		onCancel: VoidFunction;
	};

	const { title, content, open, onConfirm, onCancel }: Props = $props();
</script>

<Dialog.Root
	{open}
	onOpenChange={(open) => {
		if (!open) onCancel();
	}}>
	<Dialog.Portal>
		<Dialog.Overlay>
			{#snippet child({ props, open })}
				{#if open}
					<div {...props} transition:fade={{ duration: 300 }}></div>
				{/if}
			{/snippet}
		</Dialog.Overlay>
		<Dialog.Content>
			{#snippet child({ props, open })}
				{#if open}
					<div {...props} transition:scale={{ duration: 300, start: 0.95 }}>
						<div class="dialog__header"><h3>{title}</h3></div>

						<div class="dialog__content">
							<p>{content}</p>
						</div>
						<div class="dialog__footer">
							<div class="dialog__footer__actions">
								<button class="button" onclick={onConfirm}>Confirm</button>
								<div style="flex: auto;"></div>
								<button class="button button--secondary" onclick={onCancel}>Cancel</button>
							</div>
						</div>
					</div>
				{/if}
			{/snippet}
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
