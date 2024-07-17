<script lang="ts">
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import type { EventPipeline } from '$lib/api/types';
	import { HttpMethod, requestText } from '$lib/api/utils';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { Dialog } from 'bits-ui';
	import { _ } from 'svelte-i18n';
	import { toast } from 'svelte-sonner';
	import { fly } from 'svelte/transition';

	// The pipeline to delete
	export let pipeline: EventPipeline;

	export let onClose: () => void;

	const client = useQueryClient();

	// Mutation to delete the pipeline
	$: deleteMutation = createMutation({
		mutationFn: async () =>
			await requestText({
				method: HttpMethod.DELETE,
				route: `/api/event-pipelines/${pipeline.id}`
			}),

		onSuccess: async () => {
			await goto(`${base}/pipelines`);

			client.invalidateQueries({ queryKey: ['event-pipelines'] });

			toast.success('Deleted pipeline.');

			// Clear state for the deleted pipeline
			client.removeQueries({ queryKey: ['event-pipelines', pipeline.id] });
			client.cancelQueries({ queryKey: ['event-pipelines', pipeline.id] });

			onClose();
		}
	});
</script>

<Dialog.Root
	open
	onOpenChange={(open) => {
		if (!open) onClose();
	}}
>
	<Dialog.Portal>
		<Dialog.Overlay transition={fly} transitionConfig={{ duration: 300, y: -10 }} />
		<Dialog.Content transition={fly} transitionConfig={{ duration: 300, y: -10 }}>
			<div class="dialog__header"><h3>{$_('delete_pipeline.title')}</h3></div>

			<div class="dialog__content">
				<p>{$_('delete_pipeline.message')}</p>

				{#if $deleteMutation.error !== null}
					<p class="error">{$deleteMutation.error.message}</p>
				{/if}
			</div>
			<div class="dialog__footer">
				<div class="dialog__footer__actions">
					<button
						class="button"
						disabled={$deleteMutation.isPending}
						on:click={() => $deleteMutation.mutate()}>Delete</button
					>
					<div style="flex: auto;"></div>
					<button class="button button--secondary" on:click={onClose}>Cancel</button>
				</div>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

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
