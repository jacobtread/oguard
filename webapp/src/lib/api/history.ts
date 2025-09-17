import type { Dayjs } from 'dayjs';
import type { DeviceBatteryHistory, DeviceStateHistory, EventHistory } from './types';
import { HttpMethod, requestJson } from './utils';
import { createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
import dayjs from 'dayjs';

export const DEVICE_BATTERY_HISTORY_KEY = 'device-battery-history';
export const DEVICE_STATE_HISTORY_KEY = 'device-state-history';
export const EVENT_HISTORY_KEY = 'event-history';

/**
 * Creates a request for the battery history between a
 * provided date range
 *
 * @param startDate Start of the date range
 * @param endDate End of the date range
 */
export function batteryHistoryRequest(
	startDate: Dayjs,
	endDate: Dayjs
): Promise<DeviceBatteryHistory[]> {
	const query = new URLSearchParams();
	query.set('start', startDate.toISOString());
	query.set('end', endDate.toISOString());

	return requestJson<DeviceBatteryHistory[]>({
		method: HttpMethod.GET,
		route: `/api/history/battery-state?` + query.toString()
	});
}

/**
 * Creates a query that request the battery state history
 * for the provided range
 *
 * @param start Store containing the query start date
 * @param end Store containing the query end date
 */
export function createDeviceBatteryHistoryQuery(
	start: () => Date,
	end: () => Date,
	refetchInterval?: number
): CreateQueryResult<DeviceBatteryHistory[], Error> {
	return createQuery<DeviceBatteryHistory[]>(() => {
		const $start = start();
		const $end = end();
		return {
			queryKey: [DEVICE_BATTERY_HISTORY_KEY, $start.toISOString(), $end.toISOString()],
			queryFn: () => {
				const startDate = dayjs($start).utc();
				const endDate = dayjs($end).utc();

				return batteryHistoryRequest(startDate, endDate);
			},
			refetchInterval
		};
	});
}

/**
 * Creates a request for the device state history between a
 * provided date range
 *
 * @param startDate Start of the date range
 * @param endDate End of the date range
 */
export function deviceStateHistoryRequest(
	startDate: Dayjs,
	endDate: Dayjs
): Promise<DeviceStateHistory[]> {
	const query = new URLSearchParams();
	query.set('start', startDate.toISOString());
	query.set('end', endDate.toISOString());

	return requestJson<DeviceStateHistory[]>({
		method: HttpMethod.GET,
		route: `/api/history/device-state?` + query.toString()
	});
}

/**
 * Creates a query that request the device state history
 * for the provided range
 *
 * @param start Store containing the query start date
 * @param end Store containing the query end date
 */
export function createDeviceStateHistoryQuery(
	start: () => Date,
	end: () => Date
): CreateQueryResult<DeviceStateHistory[], Error> {
	return createQuery<DeviceStateHistory[]>(() => {
		const $start = start();
		const $end = end();
		return {
			queryKey: [DEVICE_STATE_HISTORY_KEY, $start.toISOString(), $end.toISOString()],
			queryFn: () => {
				const startDate = dayjs($start).utc();
				const endDate = dayjs($end).utc();

				return deviceStateHistoryRequest(startDate, endDate);
			}
		};
	});
}

/**
 * Creates a request for the event history between a
 * provided date range
 *
 * @param startDate Start of the date range
 * @param endDate End of the date range
 */
export function eventHistoryRequest(startDate: Dayjs, endDate: Dayjs): Promise<EventHistory[]> {
	const query = new URLSearchParams();
	query.set('start', startDate.toISOString());
	query.set('end', endDate.toISOString());

	return requestJson<EventHistory[]>({
		method: HttpMethod.GET,
		route: '/api/history/event?' + query.toString()
	});
}

/**
 * Creates a query that request the event history
 * for the provided range
 *
 * @param start Store containing the query start date
 * @param end Store containing the query end date
 */
export function createEventHistoryQuery(
	start: () => Date,
	end: () => Date,
	refetchInterval?: number
): CreateQueryResult<EventHistory[], Error> {
	return createQuery<EventHistory[]>(() => {
		const $start = start();
		const $end = end();
		return {
			queryKey: [EVENT_HISTORY_KEY, $start.toISOString(), $end.toISOString()],
			queryFn: () => {
				const startDate = dayjs($start).utc();
				const endDate = dayjs($end).utc();

				return eventHistoryRequest(startDate, endDate);
			},
			refetchInterval
		};
	});
}
