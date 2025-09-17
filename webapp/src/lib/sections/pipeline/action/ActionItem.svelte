<script lang="ts">
	import type { Action } from '$lib/api/types';
	import { t } from 'svelte-i18n';
	import ActionTypeIcon from '$lib/sections/pipeline/action/ActionTypeIcon.svelte';
	import { Tooltip } from 'bits-ui';
	import dayjs from 'dayjs';

	export let index: number;
	export let item: Action & { id: string };

	export let onRemove: () => void;
	export let onEdit: () => void;
</script>

<div class="item">
	<div class="item__index">{index + 1}</div>
	<div class="item__icon"><ActionTypeIcon actionType={item.ty.type} /></div>

	<div class="item__text">
		<p class="item__label">{$t(`actions.${item.ty.type}.label`)}</p>
		<p class="item__description">{$t(`actions.${item.ty.type}.description`)}</p>

		<div class="item__flags">
			{#if item.delay !== null && (item.delay.duration !== null || item.delay.below_capacity !== null)}
				<Tooltip.Root>
					<Tooltip.Trigger><span class="item__flag">Delayed</span></Tooltip.Trigger>
					<Tooltip.Content>
						{#if item.delay.duration !== null && item.delay.below_capacity !== null}
							{$t('action.delay_explain.duration_capacity', {
								values: {
									duration: dayjs.duration(item.delay.duration.secs, 'seconds').humanize(),
									capacity: item.delay.below_capacity
								}
							})}
						{:else if item.delay.duration !== null}
							{$t('action.delay_explain.duration', {
								values: { duration: dayjs.duration(item.delay.duration.secs, 'seconds').humanize() }
							})}
						{:else if item.delay.below_capacity !== null}
							{$t('action.delay_explain.below_capacity', {
								values: { capacity: item.delay.below_capacity }
							})}
						{/if}
					</Tooltip.Content>
				</Tooltip.Root>
			{/if}

			{#if item.repeat !== null && (item.repeat.interval !== null || item.repeat.capacity_decrease !== null)}
				<Tooltip.Root>
					<Tooltip.Trigger><span class="item__flag">Repeating</span></Tooltip.Trigger>
					<Tooltip.Content>
						{#if item.repeat.interval !== null && item.repeat.capacity_decrease !== null}
							{$t('action.repeat_explain.interval_capacity', {
								values: {
									duration: dayjs.duration(item.repeat.interval.secs, 'seconds').humanize(),
									capacity: item.repeat.capacity_decrease
								}
							})}
						{:else if item.repeat.interval !== null}
							{$t('action.repeat_explain.interval', {
								values: {
									duration: dayjs.duration(item.repeat.interval.secs, 'seconds').humanize()
								}
							})}
						{:else if item.repeat.capacity_decrease !== null}
							{$t('action.repeat_explain.capacity', {
								values: { capacity: item.repeat.capacity_decrease }
							})}
						{/if}

						{#if item.repeat.limit}
							{$t('action.repeat_explain.limit', {
								values: { limit: item.repeat.limit }
							})}
						{:else}
							{$t('action.repeat_explain.no_limit')}
						{/if}
					</Tooltip.Content>
				</Tooltip.Root>
			{/if}
			{#if item.retry !== null}
				<Tooltip.Root>
					<Tooltip.Trigger><span class="item__flag">Retry</span></Tooltip.Trigger>
					<Tooltip.Content></Tooltip.Content>
				</Tooltip.Root>
			{/if}
		</div>
	</div>

	<div class="item__actions">
		<button class="button" on:click={onEdit}>Edit</button>
		<button class="button" on:click={onRemove}>Remove</button>
	</div>
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	:global(#dnd-action-dragged-el .item) {
		border: 0.1rem solid palette.$gray-300;
	}

	// Pipeline item
	.item {
		display: flex;
		gap: 0.5rem;
		padding: 1rem;
		align-items: center;
		background-color: #fff;
		height: 7rem;

		&:not(:last-child) {
			border-bottom: 0.1rem solid palette.$gray-300;
		}
	}

	.item__index {
		margin-right: 0.5rem;
		color: palette.$gray-500;
		font-size: 1rem;
	}

	// Icon wrapper
	.item__icon {
		font-size: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	// Ending action portion of the item
	.item__actions {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		flex: auto;
		gap: 0.5rem;

		.button {
			cursor: pointer;
		}
	}

	.item__text {
		margin-left: 1rem;
	}

	// Pipeline item name
	.item__label {
		font-weight: bold;
		color: palette.$gray-800;
		margin-bottom: 0.25rem;
	}

	.item__description {
		font-size: 0.9rem;
		color: palette.$gray-600;
	}

	.item__flags {
		display: flex;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.item__flag {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background-color: palette.$gray-300;
		border-radius: 0.25rem;
	}
</style>
