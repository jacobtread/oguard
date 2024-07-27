<script lang="ts">
	import { Popover, Switch } from 'bits-ui';
	import { t } from 'svelte-i18n';
	import type { Writable } from 'svelte/store';
	import { fly } from 'svelte/transition';

	import SettingsIcon from '~icons/solar/settings-bold-duotone';

	export let columnIds: string[];
	export let hiddenColumnIds: Writable<string[]>;

	function onChangeChecked(id: string, checked: boolean) {
		if (checked) {
			console.log('add');
			hiddenColumnIds.update((ids) => ids.filter((otherId) => otherId !== id));
		} else {
			console.log('remove');
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
					<label for="hide-{id}">{$t(`history.columns.${id}`)}</label>
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
