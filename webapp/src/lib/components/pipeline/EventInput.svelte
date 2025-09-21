<script lang="ts">
	import { EVENT_TYPE_DATA, EVENT_TYPES, EventType } from '$lib/api/types';
	import { Select } from 'bits-ui';
	import EventLevelIcon from './EventLevelIcon.svelte';
	import { fly } from 'svelte/transition';
	import { i18nContext } from '$lib/i18n/i18n.svelte';
	const i18n = i18nContext.get();

	interface Props {
		value?: EventType;
	}

	let { value = $bindable(EventType.ACFailure) }: Props = $props();

	const values = $derived(
		EVENT_TYPES.map((eventType) => ({
			value: eventType,
			label: i18n.f(`events.${eventType}.label`),
			description: i18n.f(`events.${eventType}.description`)
		}))
	);

	const selected = $derived(values.find((otherValue) => otherValue.value === value));
</script>

<Select.Root
	type="single"
	items={values}
	value={selected?.value}
	onValueChange={(selected) => {
		if (selected !== undefined) {
			value = selected as EventType;
		}
	}}>
	<Select.Trigger aria-label={i18n.f('event.select')}>
		<div class="event-current-item">
			<EventLevelIcon level={EVENT_TYPE_DATA[value].level} />

			{#if selected}
				{selected.label}
			{:else}
				{i18n.f('event.select')}
			{/if}
		</div>
	</Select.Trigger>
	<Select.Portal>
		<Select.Content sideOffset={8}>
			{#snippet child({ open, props, wrapperProps })}
				<div {...wrapperProps}>
					{#if open}
						<div {...props} transition:fly={{ duration: 150, y: -10 }}>
							{#each values as eventType}
								{@const typeData = EVENT_TYPE_DATA[eventType.value]}
								{#if typeData !== undefined}
									<Select.Item value={eventType.value} label={eventType.label}>
										{#snippet children({ selected })}
											<div class="event-item" class:event-item--selected={selected}>
												<div class="event-item__icon">
													<EventLevelIcon level={typeData.level} />
												</div>

												<div class="event-item__text">
													<p class="event-item__label">{eventType.label}</p>
													<p class="event-item__description">{eventType.description}</p>
												</div>

												{#if selected}
													<div class="ml-auto">&larr;</div>
												{/if}
											</div>
										{/snippet}
									</Select.Item>
								{/if}
							{:else}
								<span>{i18n.f('no_results')}</span>
							{/each}
						</div>
					{/if}
				</div>
			{/snippet}
		</Select.Content>
	</Select.Portal>
</Select.Root>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

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
