<script lang="ts">
	import type { ListEventPipeline } from '$lib/api/types';
	import { createChangeEnabledMutation } from '$lib/api/event-pipelines';

	import BoxIcon from '~icons/solar/box-bold-duotone';

	import { Label, Switch } from 'bits-ui';
	import dayjs from 'dayjs';

	import { onDestroy } from 'svelte';
	import { fly } from 'svelte/transition';

	export let index: number;
	export let item: ListEventPipeline;

	let canToggleEnabled: boolean = true;
	let toggleEnabledTimeout: number | null = null;

	const changeEnabledMutation = createChangeEnabledMutation();

	async function onChangeEnabled(enabled: boolean) {
		if (!canToggleEnabled) return;

		canToggleEnabled = false;

		try {
			await $changeEnabledMutation.mutate({
				id: item.id,
				enabled: !enabled
			});
		} catch (error) {
			console.error('Failed to update enabled', error);
		} finally {
			// Delay enabling the switch for 2 seconds
			toggleEnabledTimeout = setTimeout(() => {
				canToggleEnabled = true;
			}, 1000) as unknown as number;
		}
	}

	onDestroy(() => {
		if (toggleEnabledTimeout !== null) {
			clearTimeout(toggleEnabledTimeout);
		}
	});
</script>

<div class="item" in:fly|global={{ delay: index * 100, duration: 300, x: -10 }}>
	<div class="item__icon"><BoxIcon /></div>

	<a class="item__content" href="/pipelines/{item.id}">
		<p class="item__name">{item.name}</p>

		<span class="item__timestamps">
			{#if item.modified_at !== null}
				<p class="item__timestamp">
					Last Modified <span>{dayjs(item.modified_at).format('L LT')}</span>
				</p>
			{/if}
			{#if item.last_executed_at !== null}
				<p class="item__timestamp">
					Last executed <span>{dayjs(item.last_executed_at).format('L LT')}</span>
				</p>
			{/if}
		</span>
	</a>

	<div class="item__actions">
		<Label.Root>Enabled</Label.Root>
		<Switch.Root
			disabled={!canToggleEnabled}
			checked={item.enabled}
			onCheckedChange={() => {
				onChangeEnabled(item.enabled);
			}}>
			<Switch.Thumb />
		</Switch.Root>
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	// Pipeline item
	.item {
		display: flex;
		gap: 0.5rem;
		padding: 1rem;
		align-items: center;

		&:not(:last-child) {
			border-bottom: 0.1rem solid palette.$gray-300;
		}
	}

	// Icon wrapper
	.item__icon {
		font-size: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	// Content portion of the item
	.item__content {
		flex: auto;
		text-decoration: none;
		color: palette.$gray-600;

		&:hover {
			text-decoration: underline;
		}
	}

	// Ending action portion of the item
	.item__actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	// Pipeline item name
	.item__name {
		font-weight: bold;
		color: palette.$gray-800;
		margin-bottom: 0.25rem;
	}

	.item__timestamps {
		display: flex;
		gap: 0.5rem;
	}

	// Pipeline item timestamps list
	.item__timestamp {
		font-size: 0.9rem;
		color: palette.$gray-600;

		> span {
			background-color: palette.$gray-200;
			padding: 0.25rem 0.5rem;
			margin-left: 0.25rem;
			border-radius: 0.25rem;
		}
	}
</style>
