import { createMutation, createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
import type {
	CreateEventPipeline,
	EventPipeline,
	ListEventPipeline,
	UpdateEventPipeline
} from './types';
import { HttpMethod, queryClient, requestJson, requestText } from './utils';
import { derived, type Readable } from 'svelte/store';

export const EVENT_PIPELINES_KEY = 'event-pipelines';

/**
 * Creates a request to login using the provided
 * password
 *
 * @param password The password to login with
 */
export function updateEventPipelineRequest(
	id: number,
	data: UpdateEventPipeline
): Promise<EventPipeline> {
	return requestJson<EventPipeline, UpdateEventPipeline>({
		method: HttpMethod.PUT,
		route: `/api/event-pipelines/${id}`,
		body: data
	});
}

export function createPipelineRequest(data: CreateEventPipeline): Promise<EventPipeline> {
	return requestJson<EventPipeline, CreateEventPipeline>({
		method: HttpMethod.POST,
		route: '/api/event-pipelines',
		body: data
	});
}

export function testPipelineRequest(id: number): Promise<unknown> {
	return requestJson({
		method: HttpMethod.POST,
		route: `/api/event-pipelines/${id}/test`
	});
}

export function deletePipelineRequest(id: number): Promise<string> {
	return requestText({
		method: HttpMethod.DELETE,
		route: `/api/event-pipelines/${id}`
	});
}

export function getEventPipelineKey(pipelineId: number): [string, number] {
	return [EVENT_PIPELINES_KEY, pipelineId];
}

export function createEventPipelineQuery(
	pipelineId: Readable<number>
): CreateQueryResult<EventPipeline, Error> {
	return createQuery(
		derived([pipelineId], ([$pipelineId]) => ({
			queryKey: getEventPipelineKey($pipelineId),
			queryFn: ({ queryKey: [, pipelineId] }: { queryKey: [string, number] }) =>
				requestJson<EventPipeline>({
					method: HttpMethod.GET,
					route: `/api/event-pipelines/${pipelineId}`
				}),
			retry: false
		}))
	);
}

export function eventPipelinesRequest(): Promise<ListEventPipeline[]> {
	return requestJson<ListEventPipeline[]>({
		method: HttpMethod.GET,
		route: '/api/event-pipelines'
	});
}

export function createEventPipelinesQuery(): CreateQueryResult<ListEventPipeline[], Error> {
	return createQuery<ListEventPipeline[]>({
		queryKey: [EVENT_PIPELINES_KEY],
		queryFn: eventPipelinesRequest
	});
}

export function createUpdateEventPipelineMutation(onComplete: VoidFunction) {
	return createMutation<EventPipeline, Error, { id: number; data: UpdateEventPipeline }>({
		mutationFn: ({ id, data }) => updateEventPipelineRequest(id, data),
		onSuccess: (pipeline) => {
			onComplete();

			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });

			// Update the local state for the pipeline using the remote state
			queryClient.setQueryData([EVENT_PIPELINES_KEY, pipeline.id], pipeline);
		}
	});
}

export function createCreateEventPipelineMutation(onComplete: (pipeline: EventPipeline) => void) {
	return createMutation<EventPipeline, Error, { data: CreateEventPipeline }>({
		mutationFn: ({ data }) => createPipelineRequest(data),
		onSuccess: (pipeline) => {
			onComplete(pipeline);

			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });

			// We can preload the data for this since the server gives it back
			queryClient.setQueryData([EVENT_PIPELINES_KEY, pipeline.id], pipeline);
		}
	});
}

export function createTestEventPipelineMutation(onComplete: VoidFunction) {
	return createMutation<unknown, Error, { id: number }>({
		mutationFn: ({ id }) => testPipelineRequest(id),
		onSuccess: () => {
			onComplete();
		}
	});
}

export function createChangeEnabledMutation() {
	return createMutation<unknown, Error, { id: number; enabled: boolean }>({
		mutationFn: ({ id, enabled }) => updateEventPipelineRequest(id, { enabled }),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });
		}
	});
}

export function createDeletePipelineMutation(onComplete: VoidFunction) {
	return createMutation<unknown, Error, { id: number }>({
		mutationFn: ({ id }) => deletePipelineRequest(id),
		onSuccess: (_, { id }) => {
			onComplete();

			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });

			// Clear state for the deleted pipeline
			queryClient.removeQueries({ queryKey: [EVENT_PIPELINES_KEY, id] });
			queryClient.cancelQueries({ queryKey: [EVENT_PIPELINES_KEY, id] });
		}
	});
}
