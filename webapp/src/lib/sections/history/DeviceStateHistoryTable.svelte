<script lang="ts">
	/** eslint-disable svelte/valid-compile */

	import { createDeviceStateHistoryQuery } from '$/lib/api/history';
	import { type DeviceStateHistory } from '$lib/api/types';

	import { derived, type Readable } from 'svelte/store';
	import { fly } from 'svelte/transition';

	import {
		createRender,
		createTable,
		Render,
		Subscribe,
		type HeaderLabel
	} from 'svelte-headless-table';
	import { addHiddenColumns, addPagination, addSortBy } from 'svelte-headless-table/plugins';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';

	import LocalizedDateTime from '$lib/components/i18n/LocalizedDateTime.svelte';
	import Spinner from '$lib/components/Spinner.svelte';
	import ManageColumns from '$lib/components/table/ManageColumns.svelte';
	import Localized from '$lib/components/i18n/Localized.svelte';
	import Container from '$lib/components/container';
	import Pagination from '$lib/components/Pagination.svelte';

	export let start: Readable<Date>;
	export let end: Readable<Date>;

	const eventHistory = createDeviceStateHistoryQuery(start, end);

	const history = derived(eventHistory, ($eventHistory) => $eventHistory.data ?? []);

	const table = createTable(history, {
		sort: addSortBy({
			initialSortKeys: [{ id: 'timestamp', order: 'desc' }]
		}),
		page: addPagination({
			initialPageSize: 50
		}),
		hideColumns: addHiddenColumns({ initialHiddenColumnIds: ['device_line_type'] })
	});

	const header: HeaderLabel<DeviceStateHistory> = ({ id }) =>
		createRender(Localized, { key: `history.columns.${id}` });

	const columns = table.createColumns([
		table.column({
			id: 'input_voltage',
			header,
			accessor: (item) => item.state.input_voltage,
			cell: ({ value }) => `${value}V`
		}),
		table.column({
			id: 'output_voltage',
			header,
			accessor: (item) => item.state.output_voltage,
			cell: ({ value }) => `${value}V`
		}),
		table.column({
			id: 'output_load_percent',
			header,
			accessor: (item) => item.state.output_load_percent,
			cell: ({ value }) => `${value}%`
		}),
		table.column({
			id: 'output_frequency',
			header,
			accessor: (item) => item.state.output_frequency,
			cell: ({ value }) => `${value}Hz`
		}),
		table.column({
			id: 'battery_voltage',
			header,
			accessor: (item) => item.state.battery_voltage,
			cell: ({ value }) => `${value}V`
		}),
		table.column({
			id: 'device_power_state',
			header,
			accessor: (item) => item.state.device_power_state,
			cell: ({ value }) => `${value}`
		}),
		table.column({
			id: 'battery_low',
			header,
			accessor: (item) => item.state.battery_low,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
			plugins: {
				sort: {
					getSortValue: (value) => (value ? 1 : 0)
				}
			}
		}),
		table.column({
			id: 'fault_mode',
			header,
			accessor: (item) => item.state.fault_mode,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
			plugins: {
				sort: {
					getSortValue: (value) => (value ? 1 : 0)
				}
			}
		}),

		table.column({
			id: 'device_line_type',
			header,
			accessor: (item) => item.state.device_line_type,
			cell: ({ value }) => `${value}`
		}),
		table.column({
			id: 'battery_self_test',
			header,
			accessor: (item) => item.state.battery_self_test,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
			plugins: {
				sort: {
					getSortValue: (value) => (value ? 1 : 0)
				}
			}
		}),
		table.column({
			id: 'buzzer_control',
			header,
			accessor: (item) => item.state.buzzer_control,
			cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
			plugins: {
				sort: {
					getSortValue: (value) => (value ? 1 : 0)
				}
			}
		}),
		table.column({
			id: 'timestamp',
			header,
			accessor: (item) => item.created_at,
			cell: ({ value }) => createRender(LocalizedDateTime, { value })
		})
	]);

	const { flatColumns, headerRows, rows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const ids = flatColumns.map((c) => c.id);
	const { pageIndex, pageSize } = pluginStates.page;
	const { hiddenColumnIds } = pluginStates.hideColumns;
</script>

{#if $eventHistory.isPending}
	<Spinner />
{/if}
{#if $eventHistory.error}
	An error has occurred:
	{$eventHistory.error.message}
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
	@use '$lib/styles/palette.scss' as palette;

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
