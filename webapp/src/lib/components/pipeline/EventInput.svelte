<script lang="ts">
	import { EVENT_TYPE_DATA, EVENT_TYPES, EventType } from '$lib/api/types';
	import { Select } from 'bits-ui';
	import { t } from 'svelte-i18n';
	import EventLevelIcon from './EventLevelIcon.svelte';

	export let value: EventType = EventType.ACFailure;

	$: values = EVENT_TYPES.map((eventType) => ({
		value: eventType,
		label: $t(`events.${eventType}.label`),
		description: $t(`events.${eventType}.description`)
	}));

	$: selected = values.find((otherValue) => otherValue.value === value);
</script>

<Select.Root
	items={values}
	{selected}
	onSelectedChange={(selected) => {
		if (selected !== undefined) {
			value = selected.value;
		}
	}}>
	<Select.Trigger aria-label={$t('event.select')}>
		<div class="event-current-item">
			<EventLevelIcon level={EVENT_TYPE_DATA[value].level} />

			<Select.Value placeholder={$t('event.select')} />
		</div>
	</Select.Trigger>
	<Select.Content sideOffset={8} sameWidth={false}>
		{#each values as eventType}
			{@const typeData = EVENT_TYPE_DATA[eventType.value]}
			{#if typeData !== undefined}
				<Select.Item value={eventType.value} label={eventType.label} let:isSelected>
					<div class="event-item" class:event-item--selected={isSelected}>
						<div class="event-item__icon">
							<EventLevelIcon level={typeData.level} />
						</div>

						<div class="event-item__text">
							<p class="event-item__label">{eventType.label}</p>
							<p class="event-item__description">{eventType.description}</p>
						</div>
						<Select.ItemIndicator asChild={false}>
							<!-- <Check /> -->
							&larr;
						</Select.ItemIndicator>
					</div>
				</Select.Item>
			{/if}
		{:else}
			<span>{$t('no_results')}</span>
		{/each}
	</Select.Content>
	<Select.Input bind:value />
</Select.Root>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.event-item {
		display: flex;
		gap: 1rem;
		align-items: center;
		padding: 0.5rem 1rem;
		cursor: pointer;
		width: 100%;

		&--selected {
			background-color: palette.$gray-300;

			&:hover {
				background-color: palette.$gray-400;
			}
		}

		&__text {
			display: flex;
			flex-flow: column;
			gap: 0.25rem;
		}

		&__label {
			font-weight: bold;
		}
		&__description {
			font-size: 0.9rem;
		}
	}

	.event-current-item {
		display: inline-flex;
		gap: 0.5rem;
		align-items: center;
	}
</style>
