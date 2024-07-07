<script lang="ts">
	import { type EventHistory, EventLevel, EVENT_TYPE_DATA } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import dayjs from 'dayjs';
	import { DateInput } from 'date-picker-svelte';
	import DateIcon from '~icons/solar/calendar-date-bold-duotone';
	import { derived } from 'svelte/store';
	import InfoIcon from '~icons/solar/info-circle-bold-duotone';
	import WarningIcon from '~icons/solar/danger-triangle-bold-duotone';
	import ErrorIcon from '~icons/solar/bug-bold-duotone';
	import SuccessIcon from '~icons/solar/check-circle-bold-duotone';

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
</script>

<div class="content">
	<div class="filters">
		<div class="card date-input">
			<label class="date-input__label" for="startDate"><DateIcon /> Start </label>
			<DateInput id="startDate" timePrecision="minute" bind:value={start} />
		</div>

		<div class="card date-input">
			<label class="date-input__label" for="endDate"><DateIcon /> End</label>
			<DateInput id="endDate" timePrecision="minute" bind:value={end} />
		</div>
	</div>

	{#if $eventHistory.isPending}
		Loading...
	{/if}
	{#if $eventHistory.error}
		An error has occurred:
		{$eventHistory.error.message}
	{/if}
	{#if $eventHistory.isSuccess}
		<div>
			<table>
				<thead>
					<tr>
						<th class="column column--level">Level</th>
						<th>Type</th>
						<th>Description</th>
						<th>Timestamp</th>
					</tr>
				</thead>
				<tbody>
					{#each $eventHistory.data as row}
						{@const typeData = EVENT_TYPE_DATA[row.type]}
						{#if typeData !== undefined}
							<tr>
								<td class="column column--level">
									{#if typeData.level === EventLevel.Info}
										<span class="level level--info">
											<InfoIcon />
										</span>
									{:else if typeData.level === EventLevel.Success}
										<span class="level level--success">
											<SuccessIcon />
										</span>
									{:else if typeData.level === EventLevel.Warning}
										<span class="level level--warning">
											<WarningIcon />
										</span>
									{:else if typeData.level === EventLevel.Severe}
										<span class="level level--severe">
											<ErrorIcon />
										</span>
									{/if}
								</td>
								<td class="column column--type">{typeData.label}</td>
								<td class="column column--description">{typeData.description}</td>

								<td>{dayjs(row.created_at).format('L LT')}</td>
							</tr>
						{/if}
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style lang="scss">
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

	.level {
		font-size: 1.25rem;
		line-height: 1;

		&--info {
			color: #34495e;
		}

		&--success {
			color: #30b455;
		}

		&--warning {
			color: #efaf13;
		}

		&--severe {
			color: #aa1109;
		}
	}

	table {
		font-family: Arial, Helvetica, sans-serif;
		border-collapse: collapse;
		width: 100%;
		border: 1px solid #ddd;
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
		background-color: #34495e;
		color: white;
	}

	.content {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		flex-flow: column;
	}

	.filters {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
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
