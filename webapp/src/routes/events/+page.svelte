<script lang="ts">
	import { type EventHistory, EVENT_TYPE_DATA, EventLevel } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import dayjs from 'dayjs';
	import { DateInput } from 'date-picker-svelte';
	import DateIcon from '~icons/solar/calendar-date-bold-duotone';
	import { derived, writable } from 'svelte/store';
	import { t } from 'svelte-i18n';
	import { Container } from '$lib/components';
	import Breadcrumbs from '$/lib/components/Breadcrumbs.svelte';
	import { fly } from 'svelte/transition';
	import Spinner from '$/lib/components/Spinner.svelte';
	import EventLevelIcon from '$/lib/components/pipeline/EventLevelIcon.svelte';
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
	import Localized from '$/lib/components/Localized.svelte';
	import LocalizedDateTime from '$/lib/components/LocalizedDateTime.svelte';
	import Pagination from '$/lib/components/Pagination.svelte';
	import ManageColumns from '$/lib/components/table/ManageColumns.svelte';

	const currentDate = dayjs();

	let start = writable(currentDate.startOf('month').toDate());
	let end = writable(currentDate.endOf('month').toDate());

	const eventHistory = createQuery<EventHistory[]>(
		derived([start, end], ([$start, $end]) => ({
			queryKey: ['event-history', $start.toISOString(), $end.toISOString()],
			queryFn: async () => {
				const startDate = dayjs($start).utc();
				const endDate = dayjs($end).utc();

				const query = new URLSearchParams();
				query.set('start', startDate.toISOString());
				query.set('end', endDate.toISOString());
				return await requestJson<EventHistory[]>({
					method: HttpMethod.GET,
					route: '/api/history/event?' + query.toString()
				});
			},

			// Refetch the data every minute
			refetchInterval: 1000 * 60
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

	const header: HeaderLabel<EventHistory> = ({ id }) =>
		createRender(Localized, { key: `event.columns.${id}` });

	const columns = table.createColumns([
		table.column({
			id: 'level',
			header,
			accessor: (item) => EVENT_TYPE_DATA[item.type]?.level ?? EventLevel.Info,
			cell: ({ value }) => createRender(EventLevelIcon, { level: value })
		}),
		table.column({
			id: 'type',
			header,
			accessor: (item) => item.type,
			cell: ({ value }) => createRender(Localized, { key: `events.${value}.label` })
		}),
		table.column({
			id: 'description',
			header,
			accessor: (item) => item.type,
			cell: ({ value }) => createRender(Localized, { key: `events.${value}.description` }),

			plugins: {
				sort: {
					disable: true
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

<Container.Wrapper>
	<Breadcrumbs parts={[{ label: $t('pages.events') }]} />

	<Container.Root>
		<Container.Header title={$t('pages.events')}></Container.Header>
		<Container.Section>
			<div class="top-filters">
				<div class=" date-input">
					<label class="date-input__label" for="startDate">
						<DateIcon />
						{$t('event.filters.start')}
					</label>
					<DateInput id="startDate" timePrecision="minute" bind:value={$start} />
				</div>

				<div class=" date-input">
					<label class="date-input__label" for="endDate">
						<DateIcon />
						{$t('event.filters.end')}
					</label>
					<DateInput id="endDate" timePrecision="minute" bind:value={$end} />
				</div>
			</div>
		</Container.Section>

		<Container.Section indent>
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
						<ManageColumns translateKey="event.columns" columnIds={ids} {hiddenColumnIds} />
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
												class:sorted={props.sort.order !== undefined}
												class:column--level={cell.id === 'level'}>
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
											<td
												{...attrs}
												class:sorted={props.sort.order !== undefined}
												class:column--level={cell.id === 'level'}>
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
		</Container.Section>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

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
