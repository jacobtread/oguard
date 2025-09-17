import { createMutation, createQuery, QueryClient } from '@tanstack/svelte-query';
import type { DeviceBattery, DeviceState } from './types';
import { HttpMethod, queryClient, requestJson, requestText } from './utils';

export const DEVICE_STATE_KEY: [string] = ['device-state'];
export const BATTERY_INFO_KEY: [string] = ['battery-info'];

/**
 * Creates a request that starts a battery test
 */
export function startBatteryTestRequest(): Promise<string> {
	return requestText({
		method: HttpMethod.POST,
		route: '/api/test-battery/start'
	});
}

/**
 * Mutation to start a battery test
 */
export function createStartBatteryTestMutation() {
	return createMutation(() => ({
		mutationFn: startBatteryTestRequest,
		onMutate: async () => {
			await queryClient.cancelQueries({ queryKey: DEVICE_STATE_KEY });

			const previousState = updateDeviceState(queryClient, (previousState) => ({
				...previousState,
				battery_self_test: true
			}));

			return previousState;
		},

		onSettled: () => {
			queryClient.invalidateQueries({ queryKey: DEVICE_STATE_KEY });
		}
	}));
}

/**
 * Creates a request to cancel a battery test
 */
export function cancelBatteryTestRequest(): Promise<string> {
	return requestText({
		method: HttpMethod.POST,
		route: '/api/test-battery/cancel'
	});
}

/**
 * Mutation to cancel a battery test
 */
export function createCancelBatteryTestMutation() {
	return createMutation(() => ({
		mutationFn: cancelBatteryTestRequest,
		onMutate: async () => {
			await queryClient.cancelQueries({ queryKey: DEVICE_STATE_KEY });

			const previousState = updateDeviceState(queryClient, (previousState) => ({
				...previousState,
				battery_self_test: false
			}));

			return previousState;
		},

		onSettled: () => {
			queryClient.invalidateQueries({ queryKey: DEVICE_STATE_KEY });
		}
	}));
}

/**
 * Creates a request to get the current device state
 */
export function deviceStateRequest(): Promise<DeviceState> {
	return requestJson<DeviceState>({
		method: HttpMethod.GET,
		route: '/api/device-state'
	});
}

/**
 * Creates a request to get the current device battery info
 */
export function batteryInfoRequest(): Promise<DeviceBattery> {
	return requestJson<DeviceBattery>({
		method: HttpMethod.GET,
		route: '/api/battery-state'
	});
}

/**
 * Query to get the device battery at a fixed refetch interval
 *
 * @param refetchInterval The interval between fetches
 */
export function createBatteryInfoPollingQuery(refetchInterval: number) {
	return createQuery<DeviceBattery>(() => ({
		queryKey: ['battery-info'],
		queryFn: batteryInfoRequest,
		refetchInterval
	}));
}

/**
 * Query to get the device state at a fixed refetch interval
 *
 * @param refetchInterval The interval between fetches
 */
export function createDeviceStatePollingQuery(refetchInterval: number) {
	return createQuery<DeviceState>(() => ({
		queryKey: DEVICE_STATE_KEY,
		queryFn: deviceStateRequest,
		refetchInterval
	}));
}

/**
 * Mutation helper to update the cached device state
 *
 * @param client The query client
 * @param action The action to update the cached device state
 * @returns The device state if one was cached
 */
export function updateDeviceState(
	client: QueryClient,
	action: (deviceState: DeviceState) => DeviceState
) {
	const previousState = client.getQueryData<DeviceState>([DEVICE_STATE_KEY]);

	if (previousState) {
		client.setQueryData<DeviceState>(DEVICE_STATE_KEY, action(previousState));
	}

	return previousState;
}

/**
 * Mutation to toggle the device buzzer state
 */
export function createToggleBuzzerMutation() {
	return createMutation(() => ({
		mutationFn: async () =>
			await requestText({
				method: HttpMethod.POST,
				route: '/api/toggle-buzzer'
			}),

		onMutate: async () => {
			await queryClient.cancelQueries({ queryKey: DEVICE_STATE_KEY });

			const previousState = updateDeviceState(queryClient, (previousState) => ({
				...previousState,
				buzzer_control: !previousState.buzzer_control
			}));

			return previousState;
		},

		onSettled: () => {
			queryClient.invalidateQueries({ queryKey: DEVICE_STATE_KEY });
		}
	}));
}
