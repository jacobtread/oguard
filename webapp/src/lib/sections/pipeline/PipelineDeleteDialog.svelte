<script lang="ts">
	import { createDeletePipelineMutation } from '$/lib/api/event-pipelines';
	import type { EventPipeline, PipelineId } from '$lib/api/types';

	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { Dialog } from 'bits-ui';
	import { t } from 'svelte-i18n';
	import { toast } from 'svelte-sonner';
	import { fade, scale } from 'svelte/transition';

	// The pipeline to delete
	export let pipeline: EventPipeline;

	export let open: boolean;

	export let onClose: () => void;

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

			await goto(`${base}/pipelines`);
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
		<Dialog.Overlay transition={fade} transitionConfig={{ duration: 300 }} />
		<Dialog.Content transition={scale} transitionConfig={{ duration: 300, start: 0.95 }}>
			<div class="dialog__header"><h3>{$t('delete_pipeline.title')}</h3></div>

			<div class="dialog__content">
				<p>{$t('delete_pipeline.message')}</p>

				{#if deleteMutation.error !== null}
					<p class="error">{deleteMutation.error.message}</p>
				{/if}
			</div>
			<div class="dialog__footer">
				<div class="dialog__footer__actions">
					<button
						class="button"
						disabled={deleteMutation.isPending}
						on:click={() => doDelete(pipeline.id)}>Delete</button>
					<div style="flex: auto;"></div>
					<button class="button button--secondary" on:click={onClose}>Cancel</button>
				</div>
			</div>
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
