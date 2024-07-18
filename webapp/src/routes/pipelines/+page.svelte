<script lang="ts">
	import type { ListEventPipeline } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import PipelineItem from '$lib/components/pipeline/PipelineItem.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import { Container } from '$lib/components';
	import { _ } from 'svelte-i18n';

	const eventPipelinesQuery = createQuery<ListEventPipeline[]>({
		queryKey: ['event-pipelines'],
		queryFn: async () =>
			await requestJson<ListEventPipeline[]>({
				method: HttpMethod.GET,
				route: '/api/event-pipelines'
			})
	});
</script>

<Container.Wrapper>
	<Breadcrumbs parts={[{ label: 'Event Pipelines' }]} />

	<Container.Root>
		<Container.Header title={$_('actions_title')}>
			<a class="button" href="/pipelines/create">{$_('create')}</a>
		</Container.Header>

		<div class="content">
			<div class="items">
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
								{$_('pipelines.empty')}
							</p>
						</div>
					{/each}
				{/if}
			</div>
		</div>

		<Container.Footer></Container.Footer>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '../../lib/styles/palette.scss' as palette;

	.items {
		background-color: #fff;
		border: 0.1rem solid palette.$gray-300;
	}

	.content {
		padding: 1rem;
		background-color: palette.$gray-200;
	}

	.empty {
		padding: 1rem;

		&__text {
			color: palette.$gray-800;
		}
	}
</style>
