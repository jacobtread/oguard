<script lang="ts">
	import { type EventHistory, EVENT_TYPE_DATA } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import dayjs from 'dayjs';
	import { DateInput } from 'date-picker-svelte';
	import DateIcon from '~icons/solar/calendar-date-bold-duotone';
	import { derived } from 'svelte/store';
	import { t } from 'svelte-i18n';
	import { Container } from '$lib/components';
	import Breadcrumbs from '$/lib/components/Breadcrumbs.svelte';
	import { fly } from 'svelte/transition';
	import Spinner from '$/lib/components/Spinner.svelte';
	import { orderBy } from 'lodash';
	import EventLevelIcon from '$/lib/components/pipeline/EventLevelIcon.svelte';

	const currentDate = dayjs();

	let start = currentDate.startOf('month').toDate();
	let end = currentDate.endOf('month').toDate();

	$: eventHistory = createQuery<EventHistory[]>(
		derived([], () => ({
			queryKey: ['event-history'],
			queryFn: async () => {
				const startDate = dayjs(start).utc();
				const endDate = dayjs(end).utc();

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

	$: orderedHistory =
		$eventHistory.data !== undefined ? getSortedHistory($eventHistory.data) : undefined;

	function getSortedHistory(events: EventHistory[]) {
		return orderBy(events, (event) => event.created_at, 'desc');
	}
</script>

<Container.Wrapper>
	<Breadcrumbs parts={[{ label: $t('pages.events') }]} />

	<Container.Root>
		<Container.Header title={$t('pages.events')}></Container.Header>
		<Container.Section>
			<div class="filters">
				<div class=" date-input">
					<label class="date-input__label" for="startDate">
						<DateIcon />
						{$t('event.filters.start')}
					</label>
					<DateInput id="startDate" timePrecision="minute" bind:value={start} />
				</div>

				<div class=" date-input">
					<label class="date-input__label" for="endDate">
						<DateIcon />
						{$t('event.filters.end')}
					</label>
					<DateInput id="endDate" timePrecision="minute" bind:value={end} />
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
			{#if $eventHistory.isSuccess && orderedHistory !== undefined}
				<div class="events">
					<table>
						<thead>
							<tr>
								<th class="column column--level">{$t('event.columns.level')}</th>
								<th>{$t('event.columns.type')}</th>
								<th>{$t('event.columns.description')}</th>
								<th>{$t('event.columns.timestamp')}</th>
							</tr>
						</thead>
						<tbody>
							{#each orderedHistory as row, index (index)}
								{@const typeData = EVENT_TYPE_DATA[row.type]}
								{#if typeData !== undefined}
									<tr in:fly|global={{ delay: index * 25, duration: 100, x: -10 }}>
										<td class="column column--level">
											<EventLevelIcon level={typeData.level} />
										</td>

										<td class="column column--type">{$t(`events.${row.type}.label`)}</td>
										<td class="column column--description"
											>{$t(`events.${row.type}.description`)}</td>

										<td>{dayjs(row.created_at).format('L LT')}</td>
									</tr>
								{/if}
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
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
