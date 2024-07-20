<script lang="ts">
	import { EVENT_TYPE_DATA, EVENT_TYPES, type EventType } from '$lib/api/types';
	import { Combobox } from 'bits-ui';
	import { t } from 'svelte-i18n';
	import EventLevelIcon from './EventLevelIcon.svelte';

	export let value: EventType;

	let inputValue = '';
	let touchedInput = false;

	$: values = EVENT_TYPES.map((eventType) => ({
		value: eventType,
		label: $t(`events.${eventType}.label`),
		description: $t(`events.${eventType}.description`)
	}));

	$: filteredValues =
		inputValue && touchedInput
			? values.filter((value) => value.label.toLowerCase().includes(inputValue.toLowerCase()))
			: values;
</script>

<Combobox.Root items={filteredValues} bind:inputValue bind:touchedInput>
	<Combobox.Input placeholder="Select an event" bind:value></Combobox.Input>
	<Combobox.Label id="eventType" />

	<Combobox.Content sideOffset={8} sameWidth={false} class="flex flex-col gap-4">
		{#each filteredValues as eventType}
			{@const typeData = EVENT_TYPE_DATA[eventType.value]}
			{#if typeData !== undefined}
				<Combobox.Item value={eventType.value}>
					<Combobox.ItemIndicator />
					<div class="event-item">
						<div class="event-item__icon">
							<EventLevelIcon level={typeData.level} />
						</div>

						<div class="event-item__text">
							<p class="event-item__label">{eventType.label}</p>
							<p class="event-item__description">{eventType.description}</p>
						</div>
					</div>
				</Combobox.Item>
			{/if}
		{:else}
			<span> No results found </span>
		{/each}
	</Combobox.Content>
	<Combobox.Arrow />
	<Combobox.HiddenInput bind:value />
</Combobox.Root>

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

	.event-item {
		display: flex;
		gap: 1rem;
		align-items: center;
		padding: 0.5rem 1rem;

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
</style>
