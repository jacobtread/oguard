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
	level: EventLevel;
};

export const EVENT_TYPE_DATA: Record<EventType, EventTypeData> = {
	[EventType.ACFailure]: {
		level: EventLevel.Severe
	},
	[EventType.ACRecovery]: {
		level: EventLevel.Success
	},
	[EventType.UPSFault]: {
		level: EventLevel.Success
	},
	[EventType.LowBatteryModeStart]: {
		level: EventLevel.Success
	},
	[EventType.LowBatteryModeEnd]: {
		level: EventLevel.Success
	},
	[EventType.BatteryTestStart]: {
		level: EventLevel.Success
	},
	[EventType.BatteryTestEnd]: {
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
