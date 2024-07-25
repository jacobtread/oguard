<script lang="ts">
	import type { ListEventPipeline } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import PipelineItem from '$lib/components/pipeline/PipelineItem.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import { Container } from '$lib/components';
	import { t } from 'svelte-i18n';

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
	<Breadcrumbs parts={[{ label: $t('pages.pipelines') }]} />

	<Container.Root>
		<Container.Header title={$t('pages.pipelines')}>
			<a class="button" href="/pipelines/create">{$t('create')}</a>
		</Container.Header>

		<Container.Content>
			<Container.Section indent>
				<Container.Root>
					{#if $eventPipelinesQuery.isPending}
						Loading...
					{:else if $eventPipelinesQuery.error}
						An error has occurred:
						{$eventPipelinesQuery.error.message}
					{:else if $eventPipelinesQuery.isSuccess}
						{#each $eventPipelinesQuery.data as row, i (row.id)}
							<PipelineItem index={i} item={row} />
						{:else}
							<div class="empty">
								<p class="empty__text">
									{$t('pipelines.empty')}
								</p>
							</div>
						{/each}
					{/if}
				</Container.Root>
			</Container.Section>
		</Container.Content>

		<Container.Footer></Container.Footer>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.empty {
		padding: 1rem;

		&__text {
			color: palette.$gray-800;
		}
	}
</style>
