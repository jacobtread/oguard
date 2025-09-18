<script lang="ts">
	import { createDeviceStateHistoryQuery } from '$lib/api/history';
	import { type DeviceStateHistory } from '$lib/api/types';

	import { toStore } from 'svelte/store';
	import { fly } from 'svelte/transition';

	import { createTable, Render, Subscribe } from '@humanspeak/svelte-headless-table';
	import {
		addHiddenColumns,
		addPagination,
		addSortBy
	} from '@humanspeak/svelte-headless-table/plugins';

	import SortDesc from '~icons/solar/alt-arrow-down-bold';
	import SortAsc from '~icons/solar/alt-arrow-up-bold';

	import Spinner from '$lib/components/Spinner.svelte';
	import ManageColumns from '$lib/components/table/ManageColumns.svelte';
	import Container from '$lib/components/container';
	import Pagination from '$lib/components/Pagination.svelte';
	import { i18nContext } from '$lib/i18n/i18n.svelte';

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

	const table = createTable(
		toStore(() => history),
		{
			sort: addSortBy({
				initialSortKeys: [{ id: 'timestamp', order: 'desc' }]
			}),
			page: addPagination({
				initialPageSize: 50
			}),
			hideColumns: addHiddenColumns({ initialHiddenColumnIds: ['device_line_type'] })
		}
	);

	const columns = $derived(
		table.createColumns([
			table.column({
				id: 'input_voltage',
				header: i18n.f('history.columns.input_voltage'),
				accessor: (item: DeviceStateHistory) => item.state.input_voltage,
				cell: ({ value }) => `${value}V`
			}),
			table.column({
				id: 'output_voltage',
				header: i18n.f('history.columns.output_voltage'),
				accessor: (item: DeviceStateHistory) => item.state.output_voltage,
				cell: ({ value }) => `${value}V`
			}),
			table.column({
				id: 'output_load_percent',
				header: i18n.f('history.columns.output_load_percent'),
				accessor: (item: DeviceStateHistory) => item.state.output_load_percent,
				cell: ({ value }) => `${value}%`
			}),
			table.column({
				id: 'output_frequency',
				header: i18n.f('history.columns.output_frequency'),
				accessor: (item: DeviceStateHistory) => item.state.output_frequency,
				cell: ({ value }) => `${value}Hz`
			}),
			table.column({
				id: 'battery_voltage',
				header: i18n.f('history.columns.battery_voltage'),
				accessor: (item: DeviceStateHistory) => item.state.battery_voltage,
				cell: ({ value }) => `${value}V`
			}),
			table.column({
				id: 'device_power_state',
				header: i18n.f('history.columns.device_power_state'),
				accessor: (item: DeviceStateHistory) => item.state.device_power_state,
				cell: ({ value }) => `${value}`
			}),
			table.column({
				id: 'battery_low',
				header: i18n.f('history.columns.battery_low'),
				accessor: (item: DeviceStateHistory) => item.state.battery_low,
				cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
				plugins: {
					sort: {
						getSortValue: (value: boolean) => (value ? 1 : 0)
					}
				}
			}),
			table.column({
				id: 'fault_mode',
				header: i18n.f('history.columns.fault_mode'),
				accessor: (item: DeviceStateHistory) => item.state.fault_mode,
				cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
				plugins: {
					sort: {
						getSortValue: (value: boolean) => (value ? 1 : 0)
					}
				}
			}),

			table.column({
				id: 'device_line_type',
				header: i18n.f('history.columns.device_line_type'),
				accessor: (item: DeviceStateHistory) => item.state.device_line_type,
				cell: ({ value }) => `${value}`
			}),
			table.column({
				id: 'battery_self_test',
				header: i18n.f('history.columns.battery_self_test'),
				accessor: (item: DeviceStateHistory) => item.state.battery_self_test,
				cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
				plugins: {
					sort: {
						getSortValue: (value: boolean) => (value ? 1 : 0)
					}
				}
			}),
			table.column({
				id: 'buzzer_control',
				header: i18n.f('history.columns.buzzer_control'),
				accessor: (item: DeviceStateHistory) => item.state.buzzer_control,
				cell: ({ value }) => `${value ? 'Yes' : 'No'}`,
				plugins: {
					sort: {
						getSortValue: (value) => (value ? 1 : 0)
					}
				}
			}),
			table.column({
				id: 'timestamp',
				header: i18n.f('history.columns.timestamp'),
				accessor: (item: DeviceStateHistory) => item.created_at,
				cell: ({ value }) => dayjs(value).format('L LT')
			})
		])
	);

	const { flatColumns, headerRows, rows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		$derived(table.createViewModel(columns));

	const ids = $derived(flatColumns.map((c) => c.id));
	const { pageIndex, pageSize } = $derived(pluginStates.page);
	const { hiddenColumnIds } = $derived(pluginStates.hideColumns);
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
									onclick={props.sort.toggle}
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
