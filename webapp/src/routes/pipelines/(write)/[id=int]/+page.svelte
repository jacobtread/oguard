<script lang="ts">
	import { page } from '$app/stores';
	import PipelineNewEditForm from '$lib/sections/pipeline/PipelineNewEditForm.svelte';
	import Spinner from '$/lib/components/Spinner.svelte';
	import { derived, type Readable } from 'svelte/store';
	import { createEventPipelineQuery } from '$/lib/api/event-pipelines';

	const pipelineId: Readable<number> = derived(page, ($page) => parseInt($page.params.id));
	const pipelineQuery = createEventPipelineQuery(pipelineId);
</script>

{#if $pipelineQuery.isPending}
	<Spinner />
{:else if $pipelineQuery.error}
	An error has occurred:
	{$pipelineQuery.error.message}
{:else if $pipelineQuery.isSuccess}
	<PipelineNewEditForm existing={$pipelineQuery.data} />
{/if}
