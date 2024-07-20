<script lang="ts">
	import {
		type EventPipeline,
		type Action,
		type UpdateEventPipeline,
		EventType,
		type CreateEventPipeline
	} from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { base } from '$app/paths';
	import ActionItem from '$lib/components/pipeline/ActionItem.svelte';
	import CreateActionForm from '$lib/components/pipeline/CreateActionForm.svelte';
	import EditActionForm from '$lib/components/pipeline/EditActionForm.svelte';
	import { Switch } from 'bits-ui';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';
	import Breadcrumbs from '../Breadcrumbs.svelte';
	import DeletePipelineDialog from './DeletePipelineDialog.svelte';
	import { Container } from '..';
	import { t } from 'svelte-i18n';
	import EventInput from './EventInput.svelte';
	import ConfirmDialog from '../ConfirmDialog.svelte';

	// Mutation arg types
	type UpdateData = { id: number; data: UpdateEventPipeline };
	type TestData = { id: number };
	type CreateData = { data: CreateEventPipeline };

	// Existing pipeline to edit if editing
	export let existing: EventPipeline | undefined = undefined;

	// Local dialog and editing state
	let confirmDelete = false;
	let confirmTest = false;
	let addAction = false;
	let editAction: number | null = null;

	// Local state for updates
	let eventType: EventType = EventType.ACFailure;
	let name: string = '';
	let cancellable: boolean = false;
	let enabled: boolean = true;
	let actions: Action[] = [];

	// Setup default state
	$: setDefaultState(existing);

	// Determine the current editing action from its index
	$: editingAction = editAction === null ? null : actions[editAction];

	const client = useQueryClient();

	// Mutation to update an existing pipeline
	const updateMutation = createMutation({
		mutationFn: async ({ id, data }: UpdateData) =>
			await requestJson<EventPipeline, UpdateEventPipeline>({
				method: HttpMethod.PUT,
				route: `/api/event-pipelines/${id}`,
				body: data
			}),

		onSuccess: (pipeline) => {
			client.invalidateQueries({ queryKey: ['event-pipelines'] });

			// Update the local state for the pipeline using the remote state
			client.setQueryData(['event-pipelines', pipeline.id], pipeline);

			toast.success('Saved changes.');
		}
	});

	// Mutation to create the pipeline
	const createPipelineMutation = createMutation({
		mutationFn: async ({ data }: CreateData) =>
			await requestJson<EventPipeline, CreateEventPipeline>({
				method: HttpMethod.POST,
				route: '/api/event-pipelines',
				body: data
			}),

		onSuccess: (pipeline) => {
			goto(`${base}/pipelines/${pipeline.id}`);

			client.invalidateQueries({ queryKey: ['event-pipelines'] });

			// We can preload the data for this since the server gives it back
			client.setQueryData(['event-pipelines', pipeline.id], pipeline);

			toast.success('Created new pipeline.');
		}
	});

	// Mutation to test an existing pipeline
	const testMutation = createMutation({
		mutationFn: async ({ id }: TestData) =>
			await requestJson({
				method: HttpMethod.POST,
				route: `/api/event-pipelines/${id}/test`
			}),

		onSuccess: () => {
			toast.success('Test started.');
		}
	});

	/**
	 * Removes the action at the provided index
	 *
	 * @param index The index of the action to remove
	 */
	function removeAction(index: number) {
		actions.splice(index, 1);
		actions = actions;
	}

	/**
	 * Sets the options and actions back to the default
	 * values, uses the values from the existing pipeline
	 * if one is provided
	 *
	 * @param existing Existing pipeline to reset to
	 */
	function setDefaultState(existing?: EventPipeline) {
		if (existing) {
			eventType = existing.event;
			name = existing.name;
			cancellable = existing.cancellable;
			enabled = existing.enabled;
			actions = [...existing.pipeline.actions];
		} else {
			eventType = EventType.ACFailure;
			name = '';
			cancellable = false;
			enabled = true;
			actions = [];
		}
	}
</script>

<Container.Wrapper>
	<Breadcrumbs
		parts={[
			{ label: 'Event Pipelines', href: `${base}/pipelines` },
			existing !== undefined ? { label: existing.name } : { label: 'Create' }
		]} />

	<Container.Root>
		<Container.Header
			title={existing !== undefined ? $t('pipelines.editing_title') : $t('pipelines.create_title')}>
			<div class="actions">
				{#if existing !== undefined}
					<span class="pipeline-name">{existing.name}</span>
				{/if}
				<a class="button button--secondary" href="{base}/pipelines">Back</a>
			</div>
		</Container.Header>

		<div class="settings">
			<div class="fls">
				<div class="fl">
					<div class="fl__text">
						<h3 class="fl__name">Event</h3>
						<p class="fl__description">Choose an event this pipeline should run on</p>
					</div>
					<EventInput bind:value={eventType} />
				</div>

				<div class="fl">
					<div class="fl__text">
						<h3 class="fl__name">Name</h3>
						<p class="fl__description">Choose a name for this pipeline</p>
					</div>
					<input class="input" id="name" type="text" required maxlength="100" bind:value={name} />
				</div>

				<div class="fl">
					<div class="fl__text">
						<h3 class="fl__name">Cancellable</h3>
						<p class="fl__description">
							When cancelling is enabled if this pipeline is running and the opposite of the event
							is received the current execution will be canceled
						</p>
					</div>

					<div>
						<Switch.Root
							checked={cancellable}
							onCheckedChange={(value) => {
								cancellable = value;
							}}>
							<Switch.Thumb />
						</Switch.Root>
					</div>
				</div>

				{#if existing !== undefined}
					<div class="fl">
						<div class="fl__text">
							<h3 class="fl__name">Enabled</h3>
							<p class="fl__description">
								Whether this event pipeline is enabled, the pipeline will not run if it is not
								enabled
							</p>
						</div>
						<div>
							<Switch.Root
								checked={enabled}
								onCheckedChange={(value) => {
									enabled = value;
								}}>
								<Switch.Thumb />
							</Switch.Root>
						</div>
					</div>
				{/if}
			</div>
		</div>

		<div class="content">
			<div class="items">
				{#each actions as action, index}
					<ActionItem
						{index}
						item={action}
						onEdit={() => (editAction = index)}
						onRemove={() => removeAction(index)} />
				{:else}
					<p class="empty">
						You don't have any actions in this pipeline, press <b>Add Action</b> to add an action
					</p>
				{/each}
			</div>
		</div>

		<Container.Footer>
			<svelte:fragment slot="actions">
				<button class="button" on:click={() => (addAction = true)}>Add Action</button>
				<div style="flex: auto;"></div>
				{#if existing !== undefined}
					<button
						class="button"
						disabled={$updateMutation.isPending}
						on:click={() => {
							$updateMutation.mutate({
								id: existing.id,
								data: {
									event: eventType,
									name,
									cancellable,
									enabled,
									pipeline: {
										actions
									}
								}
							});
						}}>
						Save
					</button>

					<button
						class="button button--secondary"
						on:click={() => {
							setDefaultState(existing);
							toast.info('Reverted changes.');
						}}>
						Reset
					</button>
					<button
						class="button button--secondary"
						on:click={() => {
							confirmTest = true;
						}}>
						Test
					</button>
					<button
						class="button button--secondary"
						on:click={() => {
							confirmDelete = true;
						}}>
						Delete
					</button>
				{:else}
					<button
						class="button"
						on:click={() => {
							$createPipelineMutation.mutate({
								data: { name, event: eventType, pipeline: { actions }, cancellable }
							});
						}}>
						Create
					</button>
				{/if}
			</svelte:fragment>
		</Container.Footer>
	</Container.Root>
</Container.Wrapper>

{#if confirmTest && existing !== undefined}
	<ConfirmDialog
		title="Confirm Test"
		content="Are you sure you want to test this pipeline? This will ignore delays and repeated actions and execute all actions you've listed including shutdown actions, it's recommend you save any changes if you have destructive actions. Only saved actions will be executed"
		onConfirm={() => {
			toast.info('Starting pipeline test..');
			$testMutation.mutate({ id: existing.id });
			confirmTest = false;
		}}
		onCancel={() => (confirmTest = false)} />
{/if}

{#if addAction}
	<CreateActionForm
		onSubmit={(action) => {
			addAction = false;
			actions.push(action);
			actions = actions;
		}}
		onCancel={() => (addAction = false)} />
{/if}

{#if editingAction !== null && editAction !== null}
	<EditActionForm
		action={editingAction}
		onSubmit={(action) => {
			if (editAction !== null) {
				actions[editAction] = action;
				actions = actions;
			}

			editAction = null;
		}}
		onCancel={() => (editAction = null)} />
{/if}

{#if confirmDelete && existing !== undefined}
	<DeletePipelineDialog pipeline={existing} onClose={() => (confirmDelete = false)} />
{/if}

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.fls {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	// Span the columns for the last child when theres an odd number of items
	.fl:nth-child(odd):last-child {
		grid-column: 1/3;
	}

	.fl {
		border: $border;
		background-color: palette.$gray-100;
		padding: 1rem;
		border-radius: 0.125rem;
		display: flex;
		gap: 1rem;
		width: 100%;
		align-items: center;

		&__text {
			flex: auto;
		}

		&__name {
			font-size: 1rem;
			margin-bottom: 0.25rem;
		}

		&__description {
			font-size: 0.9rem;
			color: palette.$gray-700;
		}
	}

	.items {
		background-color: #fff;
		border: 0.1rem solid palette.$gray-300;
	}

	.content {
		padding: 1rem;
		background-color: palette.$gray-200;
	}

	.settings {
		padding: 1rem;
		display: flex;
		flex-flow: column;
		gap: 0.5rem;
		border-bottom: $border;
	}

	.pipeline-name {
		background-color: palette.$gray-200;
		padding: 0.5rem;
		margin-left: 0.25rem;
		font-size: 0.9rem;
		border-radius: 0.25rem;
	}

	.empty {
		display: block;
		padding: 1rem;
		color: palette.$gray-800;
	}

	.actions {
		font-size: 1.25rem;
		color: palette.$gray-800;
		display: flex;
		gap: 1rem;
		align-items: center;
	}
</style>
