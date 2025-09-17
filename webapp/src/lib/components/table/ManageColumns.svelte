<script lang="ts">
	import { i18nContext } from '$/lib/i18n/i18n.svelte';
	import { Popover, Switch } from 'bits-ui';
	import type { Writable } from 'svelte/store';
	import { fly } from 'svelte/transition';

	import SettingsIcon from '~icons/solar/settings-bold-duotone';

	const i18n = i18nContext.get();

	export let translateKey: string;
	export let columnIds: string[];
	export let hiddenColumnIds: Writable<string[]>;

	function onChangeChecked(id: string, checked: boolean) {
		if (checked) {
			hiddenColumnIds.update((ids) => ids.filter((otherId) => otherId !== id));
		} else {
			hiddenColumnIds.update((ids) => [...ids, id]);
		}
	}
</script>

<Popover.Root>
	<Popover.Trigger class="button">
		<SettingsIcon />
	</Popover.Trigger>
	<Popover.Content
		transition={fly}
		transitionConfig={{ duration: 150, y: -10 }}
		sideOffset={8}
		sameWidth={false}>
		<Popover.Arrow />

		<p class="title">Manage Columns</p>

		<div class="columns">
			{#each columnIds as id}
				<div class="column">
					<Switch.Root
						id="hide-{id}"
						checked={!$hiddenColumnIds.includes(id)}
						onCheckedChange={(value) => {
							onChangeChecked(id, value);
						}}>
						<Switch.Thumb />
					</Switch.Root>
					<label for="hide-{id}">{i18n.f(`${translateKey}.${id}`)}</label>
				</div>
			{/each}
		</div>
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
