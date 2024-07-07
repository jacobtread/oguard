export interface DeviceBattery {
	capacity: number;
	remaining_time: number;
}

export enum DevicePowerState {
	Utility = 'Utility',
	Battery = 'Battery'
}

export enum DeviceLineType {
	LineInteractive = 'LineInteractive',
	OnLine = 'OnLine'
}

export interface DeviceState {
	input_voltage: number;
	output_voltage: number;
	output_load_percent: number;
	output_frequency: number;
	battery_voltage: number;
	device_power_state: DevicePowerState;
	battery_low: boolean;
	fault_mode: boolean;
	device_line_type: DeviceLineType;
	battery_self_test: boolean;
	buzzer_control: boolean;
}

export enum EventType {
	ACFailure = 'ACFailure',
	ACRecovery = 'ACRecovery',
	UPSFault = 'UPSFault',
	LowBatteryModeStart = 'LowBatteryModeStart',
	LowBatteryModeEnd = 'LowBatteryModeEnd',
	BatteryTestStart = 'BatteryTestStart',
	BatteryTestEnd = 'BatteryTestEnd'
}

export enum EventLevel {
	Info = 0,
	Success = 1,
	Warning = 2,
	Severe = 3
}

export type EventTypeData = {
	label: string;
	description: string;
	level: EventLevel;
};

export const EVENT_TYPE_DATA: Record<EventType, EventTypeData> = {
	[EventType.ACFailure]: {
		label: 'AC Power Failure',
		description: 'No longer receiving AC power, running on battery',
		level: EventLevel.Severe
	},
	[EventType.ACRecovery]: {
		label: 'AC Power Recovered',
		description: 'Receiving AC power, no longer running on battery',
		level: EventLevel.Success
	},
	[EventType.UPSFault]: {
		label: 'Fault encountered',
		description: 'UPS Encountered a fault',
		level: EventLevel.Success
	},
	[EventType.LowBatteryModeStart]: {
		label: 'UPS Low Battery',
		description: 'UPS is running low on battery',
		level: EventLevel.Success
	},
	[EventType.LowBatteryModeEnd]: {
		label: 'UPS Sufficient Battery',
		description: 'UPS is no longer running low on battery',
		level: EventLevel.Success
	},
	[EventType.BatteryTestStart]: {
		label: 'UPS Battery Test Started',
		description: 'UPS is testing the battery',
		level: EventLevel.Success
	},
	[EventType.BatteryTestEnd]: {
		label: 'UPS Battery Test Finished',
		description: 'UPS has finished testing the battery',
		level: EventLevel.Success
	}
};

export interface DeviceStateHistory {
	id: number;
	state: DeviceState;
	created_at: string;
}

export interface DeviceBatteryHistory {
	id: number;
	state: DeviceBattery;
	created_at: string;
}

export interface EventHistory {
	id: number;
	type: EventType;
	created_at: string;
}
