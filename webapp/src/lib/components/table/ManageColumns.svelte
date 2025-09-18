<script lang="ts" generics="T">
	import { i18nContext } from '$lib/i18n/i18n.svelte';
	import type { Table } from '@tanstack/table-core';
	import { Popover, Switch } from 'bits-ui';
	import { fly } from 'svelte/transition';

	import SettingsIcon from '~icons/solar/settings-bold-duotone';

	const i18n = i18nContext.get();

	interface Props {
		translateKey: string;
		table: Table<T>;
	}

	let { translateKey, table }: Props = $props();
</script>

<Popover.Root>
	<Popover.Trigger class="button">
		<SettingsIcon />
	</Popover.Trigger>
	<Popover.Content sideOffset={8}>
		{#snippet child({ open, props, wrapperProps })}
			<div {...wrapperProps}>
				{#if open}
					<div {...props} transition:fly={{ duration: 150, y: -100 }}>
						<Popover.Arrow />

						<p class="title">Manage Columns</p>

						<div class="columns">
							{#each table.getAllColumns().filter((col) => col.getCanHide()) as column (column)}
								<div class="column">
									<Switch.Root
										id="hide-{column.id}"
										checked={column.getIsVisible()}
										onCheckedChange={(value) => {
											column.toggleVisibility(value);
										}}>
										<Switch.Thumb />
									</Switch.Root>
									<label for="hide-{column.id}">{i18n.f(`${translateKey}.${column.id}`)}</label>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/snippet}
	</Popover.Content>
</Popover.Root>

<style>
	.title {
		margin-bottom: 1rem;
		font-weight: bold;
		font-size: 1rem;
	}

	.columns {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem 1rem;
	}

	.column {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
</style>
