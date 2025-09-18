<script lang="ts">
	import { createDeletePipelineMutation } from '$lib/api/event-pipelines';
	import type { EventPipeline, PipelineId } from '$lib/api/types';

	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { Dialog } from 'bits-ui';
	import { toast } from 'svelte-sonner';
	import { fade, scale } from 'svelte/transition';
	import { i18nContext } from '$lib/i18n/i18n.svelte';

	interface Props {
		// The pipeline to delete
		pipeline: EventPipeline;
		open: boolean;
		onClose: () => void;
	}

	const { pipeline, open, onClose }: Props = $props();

	const i18n = i18nContext.get();

	const deleteMutation = createDeletePipelineMutation();

	/**
	 * Deletes the provided pipeline and navigates the user
	 * back to the pipelines list after deleting
	 *
	 * @param pipelineId The pipeline ID for the pipeline to delete
	 */
	async function doDelete(pipelineId: PipelineId) {
		try {
			toast.info('Deleting pipeline..');
			await await deleteMutation.mutateAsync({ id: pipelineId });

			await goto(resolve(`/pipelines`));
			toast.success('Deleted pipeline.');
			onClose();
		} catch (err) {
			toast.error('failed to delete pipeline');
			console.error('failed to delete pipeline', err);
		}
	}
</script>

<Dialog.Root
	{open}
	onOpenChange={(open) => {
		if (!open) onClose();
	}}>
	<Dialog.Portal>
		<Dialog.Overlay>
			{#snippet child({ open, props })}
				{#if open}
					<div {...props} transition:fade={{ duration: 300 }}></div>
				{/if}
			{/snippet}
		</Dialog.Overlay>

		<Dialog.Content>
			{#snippet child({ open, props })}
				{#if open}
					<div {...props} transition:scale={{ duration: 300, start: 0.95 }}>
						<div class="dialog__header"><h3>{i18n.f('delete_pipeline.title')}</h3></div>

						<div class="dialog__content">
							<p>{i18n.f('delete_pipeline.message')}</p>

							{#if deleteMutation.error !== null}
								<p class="error">{deleteMutation.error.message}</p>
							{/if}
						</div>
						<div class="dialog__footer">
							<div class="dialog__footer__actions">
								<button
									class="button"
									disabled={deleteMutation.isPending}
									onclick={() => doDelete(pipeline.id)}>Delete</button>
								<div style="flex: auto;"></div>
								<button class="button button--secondary" onclick={onClose}>Cancel</button>
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
