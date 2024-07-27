<script lang="ts">
	/** eslint-disable svelte/valid-compile */

	import { type DeviceBatteryHistory } from '$lib/api/types';
	import { type Readable } from 'svelte/store';
	import { addHiddenColumns, addPagination, addSortBy } from 'svelte-headless-table/plugins';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';
	import { createRender, createTable, Render, Subscribe } from 'svelte-headless-table';
	import LocalizedDateTime from '../LocalizedDateTime.svelte';
	import dayjs from 'dayjs';

	export let history: Readable<DeviceBatteryHistory[]>;

	const table = createTable(history, {
		sort: addSortBy({
			initialSortKeys: [{ id: 'timestamp', order: 'desc' }]
		}),
		page: addPagination({
			initialPageSize: 50
		}),
		hideColumns: addHiddenColumns()
	});

	const columns = table.createColumns([
		table.column({
			id: 'capacity',
			header: 'Capacity',
			accessor: (item) => item.state.capacity,
			cell: ({ value }) => `${value}%`
		}),
		table.column({
			id: 'remaining_time',
			header: 'Remaining time',
			accessor: (item) => item.state.remaining_time,
			cell: ({ value }) => dayjs.duration(value, 'seconds').humanize()
		}),
		table.column({
			id: 'timestamp',
			header: 'Timestamp',
			accessor: (item) => item.created_at,
			cell: ({ value }) => createRender(LocalizedDateTime, { value })
		})
	]);

	const { flatColumns, headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const ids = flatColumns.map((c) => c.id);
	const { pageIndex, pageCount, pageSize, hasPreviousPage, hasNextPage } = pluginStates.page;
	const { hiddenColumnIds } = pluginStates.hideColumns;
	let hideForId = Object.fromEntries(ids.map((id) => [id, false]));
	$: $hiddenColumnIds = Object.entries(hideForId)
		.filter(([, hide]) => hide)
		.map(([id]) => id);
</script>

<h2>Pagination</h2>

<div>
	<button on:click={() => $pageIndex--} disabled={!$hasPreviousPage}>Previous page</button>
	{$pageIndex + 1} of {$pageCount}
	<button on:click={() => $pageIndex++} disabled={!$hasNextPage}>Next page</button>
</div>
<div style:margin-top="1rem">
	<label for="page-size">Page size</label>
	<input id="page-size" type="number" min={1} bind:value={$pageSize} />
</div>

<h2>Hidden columns</h2>

<div style:display="grid" style:grid-template-columns="repeat(3, 1fr)">
	{#each ids as id}
		<div style:display="flex" style:align-items="center" style:gap="1rem">
			<input id="hide-{id}" type="checkbox" bind:checked={hideForId[id]} />
			<label for="hide-{id}">{id}</label>
		</div>
	{/each}
</div>

<div class="history">
	<table {...$tableAttrs}>
		<thead>
			{#each $headerRows as headerRow (headerRow.id)}
				<Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
					<tr {...rowAttrs}>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
								<th
									{...attrs}
									on:click={props.sort.toggle}
									class:sorted={props.sort.order !== undefined}>
									<Render of={cell.render()} />
									{#if props.sort.order === 'asc'}
										<SortAsc />
									{:else if props.sort.order === 'desc'}
										<SortDesc />
									{/if}
								</th>
							</Subscribe>
						{/each}
					</tr>
				</Subscribe>
			{/each}
		</thead>
		<tbody {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe attrs={row.attrs()} let:attrs rowProps={row.props()}>
					<tr {...attrs}>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
								<td {...attrs} class:sorted={props.sort.order !== undefined}>
									<Render of={cell.render()} />
								</td>
							</Subscribe>
						{/each}
					</tr>
				</Subscribe>
			{/each}
		</tbody>
	</table>
</div>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	th .resizer {
		position: absolute;
		top: 0;
		bottom: 0;
		right: -4px;
		width: 8px;
		z-index: 1;
		background: rgba(200, 200, 200, 0.5);
		cursor: col-resize;
	}

	table {
		font-family: Arial, Helvetica, sans-serif;
		border-collapse: collapse;
		background-color: #fff;
		width: 100%;
	}

	thead {
		position: sticky;
		top: 0;
		left: 0;
	}

	.history {
		width: 100%;
		overflow-x: auto;
	}

	th,
	td {
		padding: 8px;
	}

	td {
		border: 1px solid #ddd;
	}

	tr:nth-child(even) {
		background-color: #f2f2f2;
	}

	tr:hover {
		background-color: #ddd;
	}

	th {
		position: relative;
		padding-top: 12px;
		padding-bottom: 12px;
		text-align: left;
		background-color: palette.$gray-700;
		color: white;
	}

	.filters {
		display: flex;
		gap: 1rem;
	}

	.date-input {
		display: flex;
		align-items: center;
		gap: 0.5rem;

		border-radius: 0.5rem;
		padding: 0.5rem;

		&__label {
			display: flex;
			align-items: center;
			gap: 0.5rem;
		}
	}
</style>
