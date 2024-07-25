import { ActionTypeKey, EventType, type ActionPipeline } from './api/types';

export type Preset = {
	name: string;
	event: EventType;
	pipeline: ActionPipeline;
	cancellable: boolean;
};

export const NOTIFY_AND_SHUTDOWN_WHEN_LOW: Preset = {
	name: 'Notify and shutdown when low battery',
	event: EventType.ACFailure,
	pipeline: {
		actions: [
			{
				ty: {
					type: ActionTypeKey.Notification
				},
				delay: null,
				repeat: null,
				retry: null
			},
			{
				ty: {
					type: ActionTypeKey.Shutdown,
					message: null,
					timeout: null,
					force_close_apps: false
				},
				delay: {
					duration: {
						secs: 5,
						nanos: 0
					},
					below_capacity: 30
				},
				repeat: null,
				retry: null
			}
		]
	},
	cancellable: false
};
