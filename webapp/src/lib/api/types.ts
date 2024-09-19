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

export const EVENT_TYPES = [
	EventType.ACFailure,
	EventType.ACRecovery,
	EventType.UPSFault,
	EventType.LowBatteryModeStart,
	EventType.LowBatteryModeEnd,
	EventType.BatteryTestStart,
	EventType.BatteryTestEnd
];

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
		level: EventLevel.Severe
	},
	[EventType.LowBatteryModeStart]: {
		level: EventLevel.Severe
	},
	[EventType.LowBatteryModeEnd]: {
		level: EventLevel.Success
	},
	[EventType.BatteryTestStart]: {
		level: EventLevel.Info
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

export enum ActionTypeKey {
	Notification = 'Notification',
	Popup = 'Popup',
	Sleep = 'Sleep',
	Shutdown = 'Shutdown',
	USPShutdown = 'USPShutdown',
	Executable = 'Executable',
	HttpRequest = 'HttpRequest'
}

export const ACTION_TYPE_KEYS: ActionTypeKey[] = [
	ActionTypeKey.Notification,
	ActionTypeKey.Popup,
	ActionTypeKey.Sleep,
	ActionTypeKey.Shutdown,
	ActionTypeKey.USPShutdown,
	ActionTypeKey.Executable,
	ActionTypeKey.HttpRequest
];

export type ActionTypeConfig<K extends ActionTypeKey> = Omit<
	Extract<ActionType, { type: K }>,
	'type'
>;

export type ActionType =
	| { type: ActionTypeKey.Notification }
	| { type: ActionTypeKey.Popup }
	| { type: ActionTypeKey.Sleep }
	| {
			type: ActionTypeKey.Shutdown;
			message: string | null;
			timeout: Duration | null;
			force_close_apps: boolean;
	  }
	| { type: ActionTypeKey.USPShutdown; delay_minutes: number }
	| { type: ActionTypeKey.Executable; exe: string; args: string[]; timeout: Duration | null }
	| {
			type: ActionTypeKey.HttpRequest;
			url: string;
			method: string;
			headers: Record<string, string>;
			body: HttpRequestBody | null;
			timeout: Duration | null;
	  };

export function getDefaultActionType(actionType: ActionTypeKey): ActionType {
	switch (actionType) {
		case ActionTypeKey.Notification:
			return { type: ActionTypeKey.Notification };
		case ActionTypeKey.Popup:
			return { type: ActionTypeKey.Popup };
		case ActionTypeKey.Sleep:
			return { type: ActionTypeKey.Sleep };
		case ActionTypeKey.Shutdown:
			return {
				type: ActionTypeKey.Shutdown,
				message: null,
				timeout: null,
				force_close_apps: false
			};
		case ActionTypeKey.USPShutdown:
			return { type: ActionTypeKey.USPShutdown, delay_minutes: 1 };
		case ActionTypeKey.Executable:
			return { type: ActionTypeKey.Executable, exe: 'notepad.exe', args: [], timeout: null };
		case ActionTypeKey.HttpRequest:
			return {
				type: ActionTypeKey.HttpRequest,
				url: 'https://example.com',
				method: 'GET',
				headers: {},
				body: null,
				timeout: null
			};
	}
}

export type HttpRequestBody = { payload: string; content_type: string };

export type Duration = {
	secs: number;
	nanos: number;
};

export type ActionDelay = {
	duration: Duration | null;
	below_capacity: number | null;
};

export type ActionRepeat = {
	interval: Duration | null;
	capacity_decrease: number | null;
	limit: number | null;
};

export enum ActionRetryDelayKey {
	Fixed = 'Fixed',
	LinearBackoff = 'LinearBackoff',
	ExponentialBackoff = 'ExponentialBackoff'
}

export function getDefaultActionRetryDelay(key: ActionRetryDelayKey): ActionRetryDelay {
	switch (key) {
		case ActionRetryDelayKey.Fixed:
			return { type: ActionRetryDelayKey.Fixed, delay: { secs: 10, nanos: 0 } };
		case ActionRetryDelayKey.LinearBackoff:
			return {
				type: ActionRetryDelayKey.LinearBackoff,
				initial: { secs: 5, nanos: 0 },
				increment: { secs: 10, nanos: 0 }
			};
		case ActionRetryDelayKey.ExponentialBackoff:
			return {
				type: ActionRetryDelayKey.ExponentialBackoff,
				initial: { secs: 5, nanos: 0 },
				exponent: 2
			};
		default:
			throw new Error('Unhandled action retry delay type');
	}
}

export const ACTION_RETRY_DELAY_KEYS: ActionRetryDelayKey[] = [
	ActionRetryDelayKey.Fixed,
	ActionRetryDelayKey.LinearBackoff,
	ActionRetryDelayKey.ExponentialBackoff
];

export type ActionRetryDelay =
	| { type: ActionRetryDelayKey.Fixed; delay: Duration }
	| { type: ActionRetryDelayKey.LinearBackoff; initial: Duration; increment: Duration }
	| { type: ActionRetryDelayKey.ExponentialBackoff; initial: Duration; exponent: number };

export type ActionRetry = {
	delay: ActionRetryDelay;
	max_attempts: number;
};

export type Action = {
	ty: ActionType;
	delay: ActionDelay | null;
	repeat: ActionRepeat | null;
	retry: ActionRetry | null;
};

export type ActionPipeline = {
	actions: Action[];
};

export type PipelineId = number;

export type ListEventPipeline = {
	id: PipelineId;
	name: string;
	event: EventType;
	cancellable: boolean;
	enabled: boolean;
	created_at: string;
	modified_at: string;
	last_executed_at: string | null;
};

export type EventPipeline = {
	id: PipelineId;
	name: string;
	event: EventType;
	pipeline: ActionPipeline;
	cancellable: boolean;
	enabled: boolean;
	created_at: string;
	modified_at: string;
	last_executed_at: string | null;
};

export type CreateEventPipeline = {
	name: string;
	event: EventType;
	pipeline: ActionPipeline;
	cancellable: boolean;
};

export type UpdateEventPipeline = Partial<{
	name: string;
	event: EventType;
	pipeline: ActionPipeline;
	cancellable: boolean;
	enabled: boolean;
}>;

export type LoginState = {
	logged_in: boolean;
};

export type LoginRequest = {
	password: string;
};
