<script lang="ts">
	import { page } from '$app/state';
	import PipelineNewEditForm from '$lib/sections/pipeline/PipelineNewEditForm.svelte';
	import Spinner from '$lib/components/Spinner.svelte';
	import { createEventPipelineQuery } from '$lib/api/event-pipelines';

	const pipelineId: number = $derived(parseInt(page.params.id!));
	const pipelineQuery = createEventPipelineQuery(() => pipelineId);
</script>

{#if pipelineQuery.isPending}
	<Spinner />
{:else if pipelineQuery.error}
	An error has occurred:
	{pipelineQuery.error.message}
{:else if pipelineQuery.isSuccess}
	<PipelineNewEditForm existing={pipelineQuery.data} />
{/if}
