<script lang="ts">
	/** eslint-disable svelte/valid-compile */

	import { type DeviceStateHistory } from '$lib/api/types';
	import { derived, type Readable } from 'svelte/store';
	import { addHiddenColumns, addPagination, addSortBy } from 'svelte-headless-table/plugins';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';
	import { createRender, createTable, Render, Subscribe } from 'svelte-headless-table';
	import LocalizedDateTime from '../LocalizedDateTime.svelte';
	import dayjs from 'dayjs';
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from '$/lib/api/utils';
	import Spinner from '../Spinner.svelte';
	import { Pagination } from 'bits-ui';

	export let start: Readable<Date>;
	export let end: Readable<Date>;

	const eventHistory = createQuery<DeviceStateHistory[]>(
		derived([start, end], ([$start, $end]) => ({
			queryKey: ['device-state-history', $start.toISOString(), $end.toISOString()],
			queryFn: async () => {
				const startDate = dayjs($start).utc();
				const endDate = dayjs($end).utc();

				const query = new URLSearchParams();
				query.set('start', startDate.toISOString());
				query.set('end', endDate.toISOString());

				return await requestJson<DeviceStateHistory[]>({
					method: HttpMethod.GET,
					route: `/api/history/device-state?` + query.toString()
				});
			}
		}))
	);

	const history = derived(eventHistory, ($eventHistory) => $eventHistory.data ?? []);

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
			id: 'input_voltage',
			header: 'input_voltage',
			accessor: (item) => item.state.input_voltage,
			cell: ({ value }) => `${value}V`
		}),
		table.column({
			id: 'output_voltage',
			header: 'output_voltage',
			accessor: (item) => item.state.output_voltage,
			cell: ({ value }) => `${value}V`
		}),
		table.column({
			id: 'output_load_percent',
			header: 'output_load_percent',
			accessor: (item) => item.state.output_load_percent,
			cell: ({ value }) => `${value}%`
		}),
		table.column({
			id: 'output_frequency',
			header: 'output_frequency',
			accessor: (item) => item.state.output_frequency,
			cell: ({ value }) => `${value}Hz`
		}),
		table.column({
			id: 'battery_voltage',
			header: 'battery_voltage',
			accessor: (item) => item.state.battery_voltage,
			cell: ({ value }) => `${value}V`
		}),
		table.column({
			id: 'device_power_state',
			header: 'device_power_state',
			accessor: (item) => item.state.device_power_state,
			cell: ({ value }) => `${value}`
		}),
		table.column({
			id: 'battery_low',
			header: 'battery_low',
			accessor: (item) => item.state.battery_low,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`
		}),
		table.column({
			id: 'fault_mode',
			header: 'fault_mode',
			accessor: (item) => item.state.fault_mode,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`
		}),

		table.column({
			id: 'device_line_type',
			header: 'device_line_type',
			accessor: (item) => item.state.device_line_type,
			cell: ({ value }) => `${value}`
		}),
		table.column({
			id: 'battery_self_test',
			header: 'battery_self_test',
			accessor: (item) => item.state.battery_self_test,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`
		}),
		table.column({
			id: 'buzzer_control',
			header: 'buzzer_control',
			accessor: (item) => item.state.buzzer_control,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`
		}),
		table.column({
			id: 'timestamp',
			header: 'Timestamp',
			accessor: (item) => item.created_at,
			cell: ({ value }) => createRender(LocalizedDateTime, { value })
		})
	]);

	const { flatColumns, headerRows, rows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const ids = flatColumns.map((c) => c.id);
	const { pageIndex, pageSize } = pluginStates.page;
	const { hiddenColumnIds } = pluginStates.hideColumns;
	let hideForId = Object.fromEntries(ids.map((id) => [id, false]));
	$: $hiddenColumnIds = Object.entries(hideForId)
		.filter(([, hide]) => hide)
		.map(([id]) => id);
</script>

{#if $eventHistory.isPending}
	<Spinner />
{/if}
{#if $eventHistory.error}
	An error has occurred:
	{$eventHistory.error.message}
{/if}

<!-- Pagination buttons -->
<Pagination.Root
	count={$rows.length}
	page={$pageIndex + 1}
	onPageChange={(page) => {
		$pageIndex = page - 1;
	}}
	perPage={$pageSize}
	let:pages>
	<!-- Pagination count selector -->
	<select bind:value={$pageSize} data-pagination-per-page-button>
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
</Pagination.Root>

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
