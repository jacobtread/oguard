import { createMutation, createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
import type {
	CreateEventPipeline,
	EventPipeline,
	ListEventPipeline,
	PipelineId,
	UpdateEventPipeline
} from './types';
import { HttpMethod, queryClient, requestJson, requestStatus } from './utils';
import { derived, type Readable } from 'svelte/store';

// Key for event pipelines
const EVENT_PIPELINES_KEY = 'event-pipelines';

// Key for an individual event pipeline
const EVENT_PIPELINE_KEY = 'event-pipeline';

/**
 * Creates a query key that refers to a specific event pipeline
 *
 * @param pipelineId The ID of the pipeline
 * @returns The query key
 */
export function getEventPipelineKey(pipelineId: number): [string, number] {
	return [EVENT_PIPELINE_KEY, pipelineId];
}

/**
 * Creates a request for a specific event pipeline
 *
 * @param id The event pipeline ID
 * @returns The event pipeline
 */
function eventPipelineRequest(id: PipelineId): Promise<EventPipeline> {
	return requestJson<EventPipeline>({
		method: HttpMethod.GET,
		route: `/api/event-pipelines/${id}`
	});
}

/**
 * Creates a query to load an event pipeline
 *
 * @param pipelineId The readable store containing the pipeline ID
 */
export function createEventPipelineQuery(
	pipelineId: Readable<number>
): CreateQueryResult<EventPipeline, Error> {
	return createQuery(
		derived([pipelineId], ([$pipelineId]) => ({
			queryKey: getEventPipelineKey($pipelineId),
			queryFn: ({ queryKey: [, pipelineId] }: { queryKey: [string, number] }) =>
				eventPipelineRequest(pipelineId),
			retry: false
		}))
	);
}

/**
 * Creates a request to query the available event pipelines
 *
 * @returns The list of event pipelines
 */
function eventPipelinesRequest(): Promise<ListEventPipeline[]> {
	return requestJson<ListEventPipeline[]>({
		method: HttpMethod.GET,
		route: '/api/event-pipelines'
	});
}

/**
 * Creates a query to load the available event pipelines
 */
export function createEventPipelinesQuery(): CreateQueryResult<ListEventPipeline[], Error> {
	return createQuery<ListEventPipeline[]>({
		queryKey: [EVENT_PIPELINES_KEY],
		queryFn: eventPipelinesRequest
	});
}

/**
 * Creates a request to update the data of an event
 * pipeline
 *
 * @param id The ID of the pipeline to update
 * @param data The data to update on the pipeline
 * @returns The updated pipeline
 */
function updateEventPipelineRequest(id: number, data: UpdateEventPipeline): Promise<EventPipeline> {
	return requestJson<EventPipeline, UpdateEventPipeline>({
		method: HttpMethod.PUT,
		route: `/api/event-pipelines/${id}`,
		body: data
	});
}

/**
 * Creates a mutation to update an event pipeline
 */
export function createUpdateEventPipelineMutation() {
	return createMutation<EventPipeline, Error, { id: number; data: UpdateEventPipeline }>({
		mutationFn: ({ id, data }) => updateEventPipelineRequest(id, data),
		onSuccess: (pipeline) => {
			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });

			// Update the local state for the pipeline using the remote state
			queryClient.setQueryData(getEventPipelineKey(pipeline.id), pipeline);
		}
	});
}

/**
 * Creates a request to create a new event pipeline
 *
 * @param data Data to create the pipeline from
 * @returns The created event pipeline
 */
function createPipelineRequest(data: CreateEventPipeline): Promise<EventPipeline> {
	return requestJson<EventPipeline, CreateEventPipeline>({
		method: HttpMethod.POST,
		route: '/api/event-pipelines',
		body: data
	});
}

/**
 * Creates a mutation to create a new event pipeline
 */
export function createCreateEventPipelineMutation() {
	return createMutation<EventPipeline, Error, { data: CreateEventPipeline }>({
		mutationFn: ({ data }) => createPipelineRequest(data),
		onSuccess: (pipeline) => {
			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });

			// We can preload the data for this since the server gives it back
			queryClient.setQueryData(getEventPipelineKey(pipeline.id), pipeline);
		}
	});
}

/**
 * Creates a request to test a pipeline
 *
 * @param id The ID of the pipeline to test
 */
function testPipelineRequest(id: PipelineId): Promise<void> {
	return requestStatus({
		method: HttpMethod.POST,
		route: `/api/event-pipelines/${id}/test`
	});
}

/**
 * Creates a mutation that will test a pipeline
 */
export function createTestEventPipelineMutation() {
	return createMutation<unknown, Error, { id: PipelineId }>({
		mutationFn: ({ id }) => testPipelineRequest(id)
	});
}

/**
 * Creates a mutation that will change the enabled state
 * of an event pipeline to the provided value
 */
export function createChangeEnabledMutation() {
	return createMutation<unknown, Error, { id: PipelineId; enabled: boolean }>({
		mutationFn: ({ id, enabled }) => updateEventPipelineRequest(id, { enabled }),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });
		}
	});
}

/**
 * Creates a request that will delete an event pipeline
 *
 * @param id The ID of the pipeline
 */
function deletePipelineRequest(id: PipelineId): Promise<void> {
	return requestStatus({
		method: HttpMethod.DELETE,
		route: `/api/event-pipelines/${id}`
	});
}

/**
 * Creates a mutation that will delete an event pipeline
 */
export function createDeletePipelineMutation() {
	return createMutation<unknown, Error, { id: PipelineId }>({
		mutationFn: ({ id }) => deletePipelineRequest(id),
		onSuccess: (_, { id }) => {
			queryClient.invalidateQueries({ queryKey: [EVENT_PIPELINES_KEY] });

			const pipelineKey = getEventPipelineKey(id);

			// Clear state for the deleted pipeline
			queryClient.removeQueries({ queryKey: pipelineKey });
			queryClient.cancelQueries({ queryKey: pipelineKey });
		}
	});
}
