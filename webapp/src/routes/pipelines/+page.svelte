<script lang="ts">
	import type { ListEventPipeline } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import PipelineItem from '$lib/components/pipeline/PipelineItem.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';

	const eventPipelinesQuery = createQuery<ListEventPipeline[]>({
		queryKey: ['event-pipelines'],
		queryFn: async () =>
			await requestJson<ListEventPipeline[]>({
				method: HttpMethod.GET,
				route: '/api/event-pipelines'
			})
	});
</script>

<div class="content">
	<Breadcrumbs parts={[{ label: 'Event Pipelines' }]} />

	<div class="actions">
		<div class="actions__header">
			<h2 class="actions__header__title">Actions</h2>

			<div class="action__buttons">
				<a class="actions-create" href="/pipelines/create">Create</a>
			</div>
		</div>
		{#if $eventPipelinesQuery.isPending}
			Loading...
		{:else if $eventPipelinesQuery.error}
			An error has occurred:
			{$eventPipelinesQuery.error.message}
		{:else if $eventPipelinesQuery.isSuccess}
			{#each $eventPipelinesQuery.data as row}
				<PipelineItem item={row} />
			{:else}
				<div class="empty">
					<p class="empty__text">
						You don't have any event pipelines press
						<a class="empty__link" href="/pipelines/create"> Create </a>
						to create a new one
					</p>
				</div>
			{/each}
		{/if}
		<div class="actions__footer"></div>
	</div>
</div>

<style lang="scss">
	@use '../../lib/styles/palette.scss' as palette;

	.actions {
		margin-top: 1rem;
		background-color: #fff;
		border: 0.1rem solid #dfe3e8;
		border-radius: 0.25rem;

		&__header {
			border-bottom: 0.1rem solid #dfe3e8;
			padding: 1rem;
			display: flex;
			justify-content: space-between;
			align-items: center;

			&__title {
				font-size: 1.25rem;
				color: palette.$gray-800;
			}
		}

		&__footer {
			border-top: 0.1rem solid #dfe3e8;
			padding: 1.5rem;
			display: flex;
			justify-content: space-between;
		}
	}

	.empty {
		padding: 1rem;

		&__text {
			color: palette.$gray-800;
		}

		&__link {
			color: palette.$gray-700;
			font-weight: bold;
		}
	}

	.content {
		padding: 1rem;
		margin: 0 auto;
		max-width: 80rem;
	}

	.actions-create {
		border: none;
		padding: 0.5rem 0.75rem;
		font-size: 0.9rem;
		border-radius: 0.25rem;
		cursor: pointer;
		background-color: palette.$gray-700;
		color: white;
		text-decoration: none;
	}
</style>
