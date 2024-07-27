<script lang="ts">
	import { type DeviceBatteryHistory } from '$lib/api/types';
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
	import { Select } from 'bits-ui';

	import DeviceBatteryHistoryTable from '$/lib/components/history/DeviceBatteryHistoryTable.svelte';

	const currentDate = dayjs();

	enum HistoryType {
		Device = 'Device',
		Battery = 'Battery'
	}

	let historyType = HistoryType.Battery;
	let start = currentDate.startOf('month').toDate();
	let end = currentDate.endOf('month').toDate();

	const eventHistory = createQuery<DeviceBatteryHistory[]>(
		derived([], () => ({
			queryKey: ['device-battery-history', start.toISOString(), end.toISOString()],
			queryFn: async () => {
				const startDate = dayjs(start).utc();
				const endDate = dayjs(end).utc();

				const query = new URLSearchParams();
				query.set('start', startDate.toISOString());
				query.set('end', endDate.toISOString());

				return await requestJson<DeviceBatteryHistory[]>({
					method: HttpMethod.GET,
					route: `/api/history/battery-state?` + query.toString()
				});
			},

			// Refetch the data every minute
			refetchInterval: 1000 * 60
		}))
	);

	$: options = [HistoryType.Battery, HistoryType.Device].map((value) => ({
		value,
		label: $t(`history.types.${value}.label`),
		description: $t(`history.types.${value}.description`)
	}));

	$: selected = options.find((value) => value.value === historyType);

	function onChangeType(option: { value: HistoryType } | undefined) {
		if (option === undefined) return;
		historyType = option.value;
	}

	const dataStore = derived(eventHistory, ($eventHistory) => $eventHistory.data ?? []);
</script>

<Container.Wrapper>
	<Breadcrumbs parts={[{ label: $t('pages.history') }]} />

	<Container.Root>
		<Container.Header title={$t('pages.history')}></Container.Header>
		<Container.Section>
			<div class="filters">
				<div class=" date-input">
					<Select.Root items={options} onSelectedChange={onChangeType} {selected}>
						<Select.Trigger>{$t(`history.types.${historyType}.label`)}</Select.Trigger>
						<Select.Content
							transition={fly}
							transitionConfig={{ duration: 150, y: -10 }}
							sideOffset={8}
							sameWidth={false}>
							{#each options as option}
								<Select.Item value={option.value} label={option.label}>
									<div class="history-type">
										<p class="history-type__label">{option.label}</p>
										<p class="history-type__description">{option.description}</p>
									</div>
								</Select.Item>
							{/each}
						</Select.Content>
						<Select.Input value={historyType} />
					</Select.Root>
				</div>

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
			{#if $eventHistory.isSuccess}
				<div class="events">
					<DeviceBatteryHistoryTable history={dataStore} />
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
