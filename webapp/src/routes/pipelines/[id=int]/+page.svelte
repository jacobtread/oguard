<script lang="ts">
	import { page } from '$app/stores';
	import type { EventPipeline, Action, UpdateEventPipeline } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { base } from '$app/paths';
	import AddActionForm from '$lib/components/pipeline/AddActionForm.svelte';

	let addAction = false;

	const client = useQueryClient();

	$: pipelineId = parseInt($page.params.id);

	$: pipelineQuery = createQuery<EventPipeline>({
		queryKey: ['player', pipelineId],
		queryFn: async () =>
			await requestJson<EventPipeline>({
				method: HttpMethod.GET,
				route: `/api/event-pipelines/${pipelineId}`
			}),
		retry: false
	});

	// Mutation to update the player details
	$: updateMutation = createMutation({
		mutationFn: async ({ name, pipeline, cancellable }: UpdateEventPipeline) =>
			await requestJson<EventPipeline, UpdateEventPipeline>({
				method: HttpMethod.PUT,
				route: `/api/event-pipelines/${pipelineId}`,
				body: { name, pipeline, cancellable }
			}),

		// Invalidate the current player details
		onSuccess: () => {
			client.invalidateQueries({ queryKey: ['event-pipelines'] });
		}
	});

	let actions: Action[] = [];

	$: {
		if ($pipelineQuery.data !== undefined) {
			actions = $pipelineQuery.data.pipeline.actions;
		}
	}
</script>

<div class="wrapper">
	<div class="container">
		{#if $pipelineQuery.isPending}
			Loading...
		{:else if $pipelineQuery.error}
			An error has occurred:
			{$pipelineQuery.error.message}
		{:else if $pipelineQuery.isSuccess}
			{@const pipeline = $pipelineQuery.data}

			<div class="container__header">
				<h2 class="title">Editing Pipeline <span class="pipeline-name">{pipeline.name}</span></h2>
			</div>
			<div class="container__content">
				{#each actions as action}
					<p>{JSON.stringify(action)}</p>
				{:else}
					<p class="empty">
						You don't have any actions in this pipeline, press <b>Add Action</b> to add an action
					</p>
				{/each}
			</div>
			<div class="container__footer">
				<div class="container__footer__actions">
					<button class="button" on:click={() => (addAction = true)}>Add Action</button>
					<div style="flex: auto;"></div>
					<button
						class="button"
						on:click={() => {
							$updateMutation.mutate({
								name: $pipelineQuery.data.name,
								cancellable: $pipelineQuery.data.cancellable,
								pipeline: {
									actions
								}
							});
						}}>Save</button
					>
					<a class="button button--secondary" href="{base}/pipelines">Back</a>
				</div>
			</div>
		{/if}
	</div>
</div>

{#if addAction}
	<AddActionForm
		onSubmit={(action) => {
			addAction = false;
			actions.push(action);
			actions = actions;
		}}
		onCancel={() => (addAction = false)}
	/>
{/if}

<style lang="scss">
	@use '../../../lib/styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.wrapper {
		padding: 1rem;
	}

	.container {
		width: 100%;
		max-width: 70rem;
		margin: 0 auto;

		background-color: #fff;
		border: $border;
		border-radius: 0.25rem;
	}

	.container__header {
		display: flex;
		padding: 1rem;

		justify-content: space-between;
		align-items: center;

		border-bottom: $border;
	}

	.container__footer {
		display: flex;
		padding: 1rem;

		justify-content: space-between;

		border-top: $border;
	}

	.container__footer__actions {
		display: flex;
		flex: auto;
		align-items: center;
		gap: 1rem;
	}

	.container__content {
	}

	.title {
		font-size: 1.25rem;
		color: palette.$gray-800;
	}

	.pipeline-name {
		background-color: palette.$gray-200;
		padding: 0.5rem;
		margin-left: 0.25rem;
		font-size: 0.9rem;
		border-radius: 0.25rem;
	}

	.empty {
		display: block;
		padding: 1rem;
		color: palette.$gray-800;
	}
</style>
