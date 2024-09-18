<script lang="ts">
	import { ActionTypeKey } from '$lib/api/types';
	import NotificationIcon from '~icons/solar/notification-unread-lines-bold-duotone';
	import DialogIcon from '~icons/solar/dialog-2-bold-duotone';
	import SleepIcon from '~icons/solar/sleeping-square-bold-duotone';
	import ShutdownIcon from '~icons/solar/power-bold-duotone';
	import BatteryIcon from '~icons/solar/battery-charge-bold-duotone';
	import TerminalIcon from '~icons/solar/programming-bold-duotone';
	import WifiIcon from '~icons/solar/wi-fi-router-minimalistic-bold-duotone';
	import type { ComponentType } from 'svelte';

	export let actionType: ActionTypeKey;

	function getActionTypeIcon(actionType: ActionTypeKey): ComponentType {
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

	const IconComponent: ComponentType = getActionTypeIcon(actionType);
</script>

<IconComponent />
