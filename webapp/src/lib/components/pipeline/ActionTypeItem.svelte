<script lang="ts">
	import { ActionTypeKey } from '$lib/api/types';
	import { _ } from 'svelte-i18n';
	import NotificationIcon from '~icons/solar/notification-unread-lines-bold-duotone';
	import DialogIcon from '~icons/solar/dialog-2-bold-duotone';
	import SleepIcon from '~icons/solar/sleeping-square-bold-duotone';
	import ShutdownIcon from '~icons/solar/power-bold-duotone';
	import BatteryIcon from '~icons/solar/battery-charge-bold-duotone';
	import TerminalIcon from '~icons/solar/programming-bold-duotone';
	import WifiIcon from '~icons/solar/wi-fi-router-minimalistic-bold-duotone';

	export let actionType: ActionTypeKey;
	export let selected: boolean;

	export let onClick: () => void;

	function getActionTypeIcon(actionType: ActionTypeKey): unknown {
		switch (actionType) {
			case ActionTypeKey.Notification:
				return NotificationIcon;
			case ActionTypeKey.Popup:
				return DialogIcon;
			case ActionTypeKey.Sleep:
				return SleepIcon;
			case ActionTypeKey.Shutdown:
				return ShutdownIcon;
			case ActionTypeKey.USPShutdown:
				return BatteryIcon;
			case ActionTypeKey.Executable:
				return TerminalIcon;
			case ActionTypeKey.HttpRequest:
				return WifiIcon;
			default:
				return NotificationIcon;
		}
	}

	const IconComponent = getActionTypeIcon(actionType);
</script>

<button
	class="item"
	class:item--selected={selected}
	data-action-type={actionType}
	on:click={onClick}
>
	<div class="item__icon"><IconComponent /></div>
	<div class="item__content">
		<p class="item__name">{$_(`actions.${actionType}.label`)}</p>
		<p class="item__description">{$_(`actions.${actionType}.description`)}</p>
	</div>
</button>

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

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
