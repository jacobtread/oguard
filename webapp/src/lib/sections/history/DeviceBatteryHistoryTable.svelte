<script lang="ts">
	/** eslint-disable svelte/valid-compile */

	import { createDeviceBatteryHistoryQuery } from '$/lib/api/history';
	import { type DeviceBatteryHistory } from '$lib/api/types';

	import { toStore } from 'svelte/store';
	import { fly } from 'svelte/transition';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';

	import { createRender, createTable, Render, Subscribe } from '@humanspeak/svelte-headless-table';
	import {
		addHiddenColumns,
		addPagination,
		addSortBy
	} from '@humanspeak/svelte-headless-table/plugins';

	import dayjs from 'dayjs';

	import LocalizedDateTime from '$lib/components/i18n/LocalizedDateTime.svelte';
	import Spinner from '$lib/components/Spinner.svelte';
	import Localized from '$lib/components/i18n/Localized.svelte';
	import ManageColumns from '$lib/components/table/ManageColumns.svelte';
	import Container from '$lib/components/container';
	import Pagination from '$lib/components/Pagination.svelte';

	type Props = {
		start: Date;
		end: Date;
	};

	const { start, end }: Props = $props();

	const eventHistory = createDeviceBatteryHistoryQuery(
		() => start,
		() => end
	);

	const history = $derived(eventHistory.data ?? []);

	const table = createTable(
		toStore(() => history),
		{
			sort: addSortBy({
				initialSortKeys: [{ id: 'timestamp', order: 'desc' }]
			}),
			page: addPagination({
				initialPageSize: 50
			}),
			hideColumns: addHiddenColumns()
		}
	);

	const header = ({ id }: { id: string }) =>
		createRender(Localized, { key: `history.columns.${id}` });

	const columns = table.createColumns([
		table.column({
			id: 'capacity',
			header,
			accessor: (item: DeviceBatteryHistory) => item.state.capacity,
			cell: ({ value }) => `${value}%`
		}),
		table.column({
			id: 'remaining_time',
			header,
			accessor: (item: DeviceBatteryHistory) => item.state.remaining_time,
			cell: ({ value }) => dayjs.duration(value, 'seconds').humanize()
		}),
		table.column({
			id: 'timestamp',
			header,
			accessor: (item: DeviceBatteryHistory) => item.created_at,
			cell: ({ value }) => createRender(LocalizedDateTime, { value })
		})
	]);

	const { flatColumns, headerRows, rows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const ids = flatColumns.map((c) => c.id);
	const { pageIndex, pageSize } = pluginStates.page;
	const { hiddenColumnIds } = pluginStates.hideColumns;
</script>

{#if eventHistory.isPending}
	<Spinner />
{/if}
{#if eventHistory.error}
	An error has occurred:
	{eventHistory.error.message}
{/if}

<div class="history">
	<Container.Root>
		<div class="filters">
			<Pagination count={$rows.length} bind:pageIndex={$pageIndex} bind:perPage={$pageSize} />
			<ManageColumns translateKey="history.columns" columnIds={ids} {hiddenColumnIds} />
		</div>
	</Container.Root>

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
			{#each $pageRows as row, index (row.id)}
				<Subscribe attrs={row.attrs()} let:attrs rowProps={row.props()}>
					<tr {...attrs} in:fly|global={{ delay: index * 25, duration: 100, x: -10 }}>
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

	<Container.Root>
		<div class="filters">
			<Pagination count={$rows.length} bind:pageIndex={$pageIndex} bind:perPage={$pageSize} />
		</div>
	</Container.Root>
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	.filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
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
		display: flex;
		flex-flow: column;
		gap: 1rem;
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
