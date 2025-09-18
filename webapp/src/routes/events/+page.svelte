<script lang="ts">
	import { type EventHistory, EVENT_TYPE_DATA, EventLevel } from '$lib/api/types';
	import dayjs from 'dayjs';
	import { DateInput } from 'date-picker-svelte';
	import DateIcon from '~icons/solar/calendar-date-bold-duotone';
	import { toStore } from 'svelte/store';
	import { Container } from '$lib/components';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import { fly } from 'svelte/transition';
	import Spinner from '$lib/components/Spinner.svelte';
	import EventLevelIcon from '$lib/components/pipeline/EventLevelIcon.svelte';
	import { createRender, createTable, Render, Subscribe } from '@humanspeak/svelte-headless-table';
	import {
		addHiddenColumns,
		addPagination,
		addSortBy
	} from '@humanspeak/svelte-headless-table/plugins';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';
	import Localized from '$lib/components/i18n/Localized.svelte';
	import LocalizedDateTime from '$lib/components/i18n/LocalizedDateTime.svelte';
	import Pagination from '$lib/components/Pagination.svelte';
	import ManageColumns from '$lib/components/table/ManageColumns.svelte';
	import { createEventHistoryQuery } from '$lib/api/history';
	import { i18nContext } from '$lib/i18n/i18n.svelte';
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
	import { createSvelteTable, FlexRender, renderSnippet } from '$lib/components/data-table';

	const i18n = i18nContext.get();

	const currentDate = dayjs();

	let start = $state(currentDate.startOf('month').toDate());
	let end = $state(currentDate.endOf('month').toDate());

	// Query the event history, refreshing the data every minute
	const eventHistory = createEventHistoryQuery(
		() => start,
		() => end,
		1000 * 60
	);

	const history = $derived(eventHistory.data ?? []);

	let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
	let sorting = $state<SortingState>([]);
	let columnFilters = $state<ColumnFiltersState>([]);
	let rowSelection = $state<RowSelectionState>({});
	let columnVisibility = $state<VisibilityState>({});

	const columnHelper = createColumnHelper<EventHistory>();

	const columns = $derived([
		columnHelper.accessor((event) => EVENT_TYPE_DATA[event.type]?.level ?? EventLevel.Info, {
			id: 'level',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: ({ getValue }) => renderSnippet(eventLevelSnippet, getValue())
		}),
		columnHelper.accessor((event) => i18n.f(`events.${event.type}.label`), {
			id: 'type',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id)
		}),
		columnHelper.accessor((event) => i18n.f(`events.${event.type}.description`), {
			id: 'description',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			enableSorting: false
		}),
		columnHelper.accessor('created_at', {
			id: 'timestamp',
			header: (cell) => renderSnippet(headerSnippet, cell.header.id),
			cell: ({ cell }) => dayjs(cell.getValue()).format('L LT')
		})
	]);

	const lotsOfHistory = $derived.by(() => {
		const h = [...history];

		for (let i = 0; i < 10; i++) {
			h.push(
				...history.map((history, index) => ({
					...history,
					created_at: dayjs(history.created_at)
						.add(i + index + 1, 'd')
						.toISOString()
				}))
			);
		}

		return h;
	});

	const table = createSvelteTable({
		get data() {
			return lotsOfHistory;
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

<svelte:head>
	<title>OGuard | {i18n.f('pages.events')}</title>
</svelte:head>

{#snippet headerSnippet(id: string)}
	{i18n.f(`event.columns.${id}`)}
{/snippet}

{#snippet eventLevelSnippet(level: EventLevel)}
	<EventLevelIcon {level} />
{/snippet}

<Container.Wrapper>
	<Breadcrumbs parts={[{ label: i18n.f('pages.events') }]} />

	<Container.Root>
		<Container.Header title={i18n.f('pages.events')}></Container.Header>
		<Container.Section>
			<div class="top-filters">
				<div class=" date-input">
					<label class="date-input__label" for="startDate">
						<DateIcon />
						{i18n.f('event.filters.start')}
					</label>
					<DateInput id="startDate" timePrecision="minute" bind:value={start} />
				</div>

				<div class=" date-input">
					<label class="date-input__label" for="endDate">
						<DateIcon />
						{i18n.f('event.filters.end')}
					</label>
					<DateInput id="endDate" timePrecision="minute" bind:value={end} />
				</div>
			</div>
		</Container.Section>

		<Container.Section indent>
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
						<Pagination
							count={table.getFilteredRowModel().rows.length}
							bind:pageIndex={
								() => pagination.pageIndex, (value) => table.setPageIndex(() => value)
							}
							bind:perPage={() => pagination.pageSize, (value) => table.setPageSize(() => value)} />
						<ManageColumns translateKey="event.columns" {table} />
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
							bind:pageIndex={
								() => pagination.pageIndex, (value) => table.setPageIndex(() => value)
							}
							bind:perPage={() => pagination.pageSize, (value) => table.setPageSize(() => value)} />
					</div>
				</Container.Root>
			</div>
		</Container.Section>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	.column {
		&--level {
			width: 70px;
			text-align: center;
		}

		&--type {
			width: 20%;
			white-space: nowrap;
		}

		&--description {
			white-space: nowrap;
		}
	}

	.history {
		display: flex;
		flex-flow: column;
		gap: 1rem;
		width: 100%;
		overflow-x: auto;
	}

	.filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
	}

	table {
		font-family: Arial, Helvetica, sans-serif;
		border-collapse: collapse;
		width: 100%;
		background-color: #fff;
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
		padding-top: 12px;
		padding-bottom: 12px;
		text-align: left;
		background-color: palette.$gray-700;
		color: white;
	}

	.top-filters {
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
