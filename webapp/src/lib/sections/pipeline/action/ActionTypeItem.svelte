<script lang="ts">
	import { ActionTypeKey } from '$lib/api/types';
	import { _ } from 'svelte-i18n';
	import ActionTypeIcon from './ActionTypeIcon.svelte';

	export let actionType: ActionTypeKey;
	export let selected: boolean;

	export let onClick: () => void;
</script>

<button
	class="item"
	class:item--selected={selected}
	data-action-type={actionType}
	on:click={onClick}>
	<div class="item__icon"><ActionTypeIcon {actionType} /></div>
	<div class="item__content">
		<p class="item__name">{$_(`actions.${actionType}.label`)}</p>
		<p class="item__description">{$_(`actions.${actionType}.description`)}</p>
	</div>
</button>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.item {
		$borderWidth: 0.1rem;
		$borderStyle: solid;
		$borderColor: #dfe3e8;
		$border: $borderWidth $borderStyle $borderColor;

		display: flex;
		width: 100%;
		align-items: center;
		text-align: left;
		gap: 0.5rem;

		padding: 0.5rem;

		background-color: palette.$gray-100;
		border: $border;
		cursor: pointer;

		border-radius: 0.2rem;
		border-radius: 0.25rem;

		transition: all 0.15s ease;
	}

	.item--selected {
		background-color: palette.$gray-700;

		.item__icon {
			color: palette.$gray-300;
		}

		.item__name {
			color: #fff;
		}
		.item__description {
			color: palette.$gray-200;
		}
	}

	.item__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.2rem;
		font-size: 1.5rem;
		color: palette.$gray-500;
	}

	.item__content {
		display: flex;
		flex-flow: column;
	}
	.item__name {
		font-weight: bold;
		font-size: 1rem;
		margin-bottom: 0.25rem;
	}

	.item__description {
		font-size: 0.9rem;
		color: palette.$gray-600;
	}
</style>
