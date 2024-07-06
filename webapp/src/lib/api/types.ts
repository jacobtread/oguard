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
