<script lang="ts">
	import dayjs from 'dayjs';
	import { DateInput } from 'date-picker-svelte';
	import DateIcon from '~icons/solar/calendar-date-bold-duotone';
	import { writable } from 'svelte/store';
	import { t } from 'svelte-i18n';
	import { Container } from '$lib/components';
	import Breadcrumbs from '$/lib/components/Breadcrumbs.svelte';
	import { fly } from 'svelte/transition';
	import { Select } from 'bits-ui';

	import DeviceBatteryHistoryTable from '$/lib/components/history/DeviceBatteryHistoryTable.svelte';
	import DeviceStateHistoryTable from '$/lib/components/history/DeviceStateHistoryTable.svelte';

	const currentDate = dayjs();

	enum HistoryType {
		Device = 'Device',
		Battery = 'Battery'
	}

	let historyType = HistoryType.Battery;

	let start = writable(currentDate.startOf('month').toDate());
	let end = writable(currentDate.endOf('month').toDate());

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
</script>

<svelte:head>
	<title>OGuard | {$t('pages.history')}</title>
</svelte:head>

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
			<div class="events">
				{#if historyType === HistoryType.Battery}
					<DeviceBatteryHistoryTable {start} {end} />
				{:else if historyType === HistoryType.Device}
					<DeviceStateHistoryTable {start} {end} />
				{/if}
			</div>
		</Container.Section>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.filters {
		display: flex;
		gap: 1rem;
	}

	.history-type {
		display: flex;
		flex-flow: column;
		gap: 0.25rem;

		padding: 0.5rem 1rem;
		cursor: pointer;
		width: 100%;

		&__label {
			font-weight: bold;
		}

		&__description {
			font-size: 0.9rem;
			color: palette.$gray-700;
		}
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
