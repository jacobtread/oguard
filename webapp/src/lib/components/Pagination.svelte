<script lang="ts">
	import { Pagination } from 'bits-ui';

	interface Props {
		count: number;
		pageIndex: number;
		perPage: number;
	}

	let { count, pageIndex = $bindable(), perPage = $bindable() }: Props = $props();

	function onPageChange(page: number) {
		pageIndex = page - 1;
	}

	$effect(() => {
		if (pageIndex * perPage > count) {
			pageIndex = 0;
		}
	});
</script>

{#if count > 0}
	<!-- Pagination buttons -->
	<Pagination.Root {count} page={pageIndex + 1} {onPageChange} {perPage}>
		{#snippet children({ pages, range })}
			<!-- Pagination count selector -->
			<select bind:value={perPage} data-pagination-per-page-button>
				<option value={5}>5</option>
				<option value={10} selected>10</option>
				<option value={20}>20</option>
				<option value={30}>30</option>
				<option value={50}>50</option>
			</select>

			<Pagination.PrevButton>Back</Pagination.PrevButton>
			<div class="pagination-pages">
				{#each pages as page (page.key)}
					{#if page.type === 'ellipsis'}
						<div data-pagination-page>...</div>
					{:else}
						<Pagination.Page {page}>
							{page.value}
						</Pagination.Page>
					{/if}
				{/each}
			</div>
			<Pagination.NextButton>Next</Pagination.NextButton>
		{/snippet}
	</Pagination.Root>
{/if}
