<script lang="ts">
	import { page } from '$app/stores';
	import type { EventPipeline } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import PipelineNewEditForm from '$lib/components/pipeline/PipelineNewEditForm.svelte';

	$: pipelineId = parseInt($page.params.id);

	$: pipelineQuery = createQuery<EventPipeline>({
		queryKey: ['event-pipelines', pipelineId],
		queryFn: async () =>
			await requestJson<EventPipeline>({
				method: HttpMethod.GET,
				route: `/api/event-pipelines/${pipelineId}`
			}),
		retry: false
	});
</script>

{#if $pipelineQuery.isPending}
	Loading...
{:else if $pipelineQuery.error}
	An error has occurred:
	{$pipelineQuery.error.message}
{:else if $pipelineQuery.isSuccess}
	<PipelineNewEditForm existing={$pipelineQuery.data} />
{/if}
