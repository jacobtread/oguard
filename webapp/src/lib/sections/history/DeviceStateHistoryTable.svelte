<script lang="ts">
	import { createDeviceStateHistoryQuery } from '$lib/api/history';
	import { type DeviceStateHistory } from '$lib/api/types';

	import { fly } from 'svelte/transition';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';

	import Spinner from '$lib/components/Spinner.svelte';
	import ManageColumns from '$lib/components/table/ManageColumns.svelte';
	import Container from '$lib/components/container';
	import Pagination from '$lib/components/Pagination.svelte';
	import { i18nContext } from '$lib/i18n/i18n.svelte';
	import { createSvelteTable, renderSnippet } from '$lib/components/data-table';
	import {
		createColumnHelper,
		getCoreRowModel,
		getFilteredRowModel,
		getPaginationRowModel,
		getSortedRowModel,
		type ColumnFiltersState,
		type PaginationState,
		type RowSelectionState,
		type SortingState,
		type VisibilityState
	} from '@tanstack/table-core';
	import dayjs from 'dayjs';
	import FlexRender from '$lib/components/data-table/flex-render.svelte';

	type Props = {
		start: Date;
		end: Date;
	};

	const { start, end }: Props = $props();
	const i18n = i18nContext.get();

	const eventHistory = createDeviceStateHistoryQuery(
		() => start,
		() => end
	);

	const history = $derived(eventHistory.data ?? []);

	let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 50 });
	let sorting = $state<SortingState>([]);
	let columnFilters = $state<ColumnFiltersState>([]);
	let rowSelection = $state<RowSelectionState>({});
	let columnVisibility = $state<VisibilityState>({});

	const columnHelper = createColumnHelper<DeviceStateHistory>();

	const columns = $derived([
		columnHelper.accessor('state.input_voltage', {
			id: 'input_voltage',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}V`
		}),
		columnHelper.accessor('state.output_voltage', {
			id: 'output_voltage',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}V`
		}),
		columnHelper.accessor('state.output_load_percent', {
			id: 'output_load_percent',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}%`
		}),
		columnHelper.accessor('state.output_frequency', {
			id: 'output_frequency',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}Hz`
		}),
		columnHelper.accessor('state.battery_voltage', {
			id: 'battery_voltage',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}V`
		}),
		columnHelper.accessor('state.device_power_state', {
			id: 'device_power_state',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}`
		}),
		columnHelper.accessor('state.battery_low', {
			id: 'battery_low',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => (cell.getValue() ? 'Yes' : 'No')
		}),
		columnHelper.accessor('state.fault_mode', {
			id: 'fault_mode',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => (cell.getValue() ? 'Yes' : 'No')
		}),
		columnHelper.accessor('state.device_line_type', {
			id: 'device_line_type',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => `${cell.getValue()}`
		}),
		columnHelper.accessor('state.battery_self_test', {
			id: 'battery_self_test',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => (cell.getValue() ? 'Yes' : 'No')
		}),
		columnHelper.accessor('state.buzzer_control', {
			id: 'buzzer_control',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: (cell) => (cell.getValue() ? 'Yes' : 'No')
		}),
		columnHelper.accessor('created_at', {
			id: 'timestamp',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: ({ cell }) => dayjs(cell.getValue()).format('L LT')
		})
	]);

	const table = createSvelteTable({
		get data() {
			return history;
		},
		get columns() {
			return columns;
		},
		state: {
			get pagination() {
				return pagination;
			},
			get sorting() {
				return sorting;
			},
			get columnVisibility() {
				return columnVisibility;
			},
			get rowSelection() {
				return rowSelection;
			},
			get columnFilters() {
				return columnFilters;
			}
		},

		getCoreRowModel: getCoreRowModel(),
		getPaginationRowModel: getPaginationRowModel(),
		getSortedRowModel: getSortedRowModel(),
		getFilteredRowModel: getFilteredRowModel(),
		onPaginationChange: (updater) => {
			if (typeof updater === 'function') {
				pagination = updater(pagination);
			} else {
				pagination = updater;
			}
		},
		onSortingChange: (updater) => {
			if (typeof updater === 'function') {
				sorting = updater(sorting);
			} else {
				sorting = updater;
			}
		},
		onColumnFiltersChange: (updater) => {
			if (typeof updater === 'function') {
				columnFilters = updater(columnFilters);
			} else {
				columnFilters = updater;
			}
		},
		onColumnVisibilityChange: (updater) => {
			if (typeof updater === 'function') {
				columnVisibility = updater(columnVisibility);
			} else {
				columnVisibility = updater;
			}
		},
		onRowSelectionChange: (updater) => {
			if (typeof updater === 'function') {
				rowSelection = updater(rowSelection);
			} else {
				rowSelection = updater;
			}
		}
	});
</script>

{#snippet headerSnippet(id: string)}
	{i18n.f(`history.columns.${id}`)}
{/snippet}

{#if eventHistory.isPending}
	<Spinner />
{:else if eventHistory.error}
	An error has occurred:
	{eventHistory.error.message}
{/if}

<div class="history">
	<Container.Root>
		<div class="filters">
			<Pagination
				count={table.getFilteredRowModel().rows.length}
				bind:pageIndex={() => pagination.pageIndex, (value) => table.setPageIndex(() => value)}
				bind:perPage={() => pagination.pageSize, (value) => table.setPageSize(() => value)} />
			<ManageColumns translateKey="history.columns" {table} />
		</div>
	</Container.Root>

	<table>
		<thead>
			{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<tr>
					{#each headerGroup.headers as header (header.id)}
						<th class="[&:has([role=checkbox])]:pl-3">
							{#if !header.isPlaceholder}
								{@const sortingState = sorting.find((item) => item.id === header.id)}
								<FlexRender
									content={header.column.columnDef.header}
									context={header.getContext()} />

								{#if sortingState}
									{#if sortingState.desc}
										<SortDesc />
									{:else}
										<SortAsc />
									{/if}
								{/if}
							{/if}
						</th>
					{/each}
				</tr>
			{/each}
		</thead>

		<tbody>
			{#each table.getRowModel().rows as row, index (row.id)}
				<tr
					data-state={row.getIsSelected() && 'selected'}
					in:fly|global={{ delay: index * 25, duration: 100, x: -10 }}>
					{#each row.getVisibleCells() as cell (cell.id)}
						<td class="[&:has([role=checkbox])]:pl-3">
							<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
						</td>
					{/each}
				</tr>
			{:else}
				<tr>
					<td colspan={columns.length} class="h-24 text-center">No results.</td>
				</tr>
			{/each}
		</tbody>
	</table>

	<Container.Root>
		<div class="filters">
			<Pagination
				count={table.getFilteredRowModel().rows.length}
				bind:pageIndex={() => pagination.pageIndex, (value) => table.setPageIndex(() => value)}
				bind:perPage={() => pagination.pageSize, (value) => table.setPageSize(() => value)} />
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
