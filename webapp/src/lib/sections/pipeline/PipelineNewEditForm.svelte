<script lang="ts">
	import {
		type EventPipeline,
		type Action,
		EventType,
		type PipelineId,
		type CreateEventPipeline,
		type UpdateEventPipeline
	} from '$lib/api/types';
	import {
		createCreateEventPipelineMutation,
		createTestEventPipelineMutation,
		createUpdateEventPipelineMutation
	} from '$/lib/api/event-pipelines';

	import { goto } from '$app/navigation';
	import { base } from '$app/paths';

	import ActionItem from '$/lib/sections/pipeline/action/ActionItem.svelte';
	import CreateActionForm from '$lib/sections/pipeline/action/CreateActionForm.svelte';
	import EditActionForm from '$lib/sections/pipeline/action/EditActionForm.svelte';
	import DeletePipelineDialog from '$/lib/sections/pipeline/PipelineDeleteDialog.svelte';

	import EventInput from '$lib/components/pipeline/EventInput.svelte';
	import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Container from '$lib/components/container';

	import { Switch } from 'bits-ui';

	import { cloneDeep, omit, uniqueId } from 'lodash';

	import { dndzone, type DndEvent } from 'svelte-dnd-action';

	import { flip } from 'svelte/animate';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-i18n';

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
	let actions: ActionWithId[] = [];

	type ActionWithId = Action & { id: string };

	function handleSort(e: CustomEvent<DndEvent<ActionWithId>>) {
		actions = e.detail.items;
	}

	// Setup default state
	$: setDefaultState(existing);

	// Determine the current editing action from its index
	$: editingAction = editAction === null ? null : actions[editAction];

	// Mutation to update an existing pipeline
	const updateMutation = createUpdateEventPipelineMutation();

	// Mutation to create the pipeline
	const createPipelineMutation = createCreateEventPipelineMutation();

	// Mutation to test an existing pipeline
	const testMutation = createTestEventPipelineMutation();

	/**
	 * Updates the data for the provided pipeline
	 *
	 * @param pipelineId The ID of the pipeline to update
	 * @param data Data to update on the pipeline
	 */
	async function doUpdatePipeline(pipelineId: PipelineId, data: UpdateEventPipeline) {
		try {
			toast.info('Saving pipeline..');
			await await updateMutation.mutateAsync({ id: pipelineId, data });

			toast.success('Saved changes.');
		} catch (err) {
			toast.error('failed to save pipeline');
			console.error('failed to save pipeline', err);
		}
	}

	/**
	 * Creates a new pipeline, navigates the user to the page
	 * for the pipeline after it is created
	 *
	 * @param data Data to create the pipeline with
	 */
	async function doCreatePipeline(data: CreateEventPipeline) {
		try {
			toast.info('Creating pipeline..');
			const pipeline = await await createPipelineMutation.mutateAsync({ data });

			toast.success('Created new pipeline.');
			goto(`${base}/pipelines/${pipeline.id}`);
		} catch (err) {
			toast.error('failed to create pipeline');
			console.error('failed to create pipeline', err);
		}
	}

	/**
	 * Runs the test pipeline
	 *
	 * @param pipelineId The test pipeline ID to run
	 */
	async function doTestMutation(pipelineId: PipelineId) {
		try {
			toast.info('Starting pipeline test..');
			await testMutation.mutateAsync({ id: pipelineId });
			toast.success('Test started.');
		} catch (err) {
			toast.error('failed to test pipeline');
			console.error('failed to test pipeline', err);
		}
	}

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
			actions = existing.pipeline.actions.map(createLocalAction);
		} else {
			eventType = EventType.ACFailure;
			name = '';
			cancellable = false;
			enabled = true;
			actions = [];
		}
	}

	function createLocalAction(action: Action): ActionWithId {
		return { ...cloneDeep(action), id: uniqueId() };
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
			<div
				class="items"
				use:dndzone={{ items: actions, flipDurationMs: 200, dropTargetStyle: {} }}
				on:consider={handleSort}
				on:finalize={handleSort}>
				{#each actions as action, index (action.id)}
					<div tabindex="0" class="item" role="button" animate:flip={{ duration: 200 }}>
						<ActionItem
							{index}
							item={action}
							onEdit={() => (editAction = index)}
							onRemove={() => removeAction(index)} />
					</div>
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
						disabled={updateMutation.isPending}
						on:click={() => {
							const pipelineActions = actions.map((action) => omit(action, 'id'));
							doUpdatePipeline(existing.id, {
								event: eventType,
								name,
								cancellable,
								enabled,
								pipeline: {
									actions: pipelineActions
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
							// Strip local ID from actions
							const pipelineActions = actions.map((action) => omit(action, 'id'));

							doCreatePipeline({
								name,
								event: eventType,
								pipeline: { actions: pipelineActions },
								cancellable
							});
						}}>
						Create
					</button>
				{/if}
			</svelte:fragment>
		</Container.Footer>
	</Container.Root>
</Container.Wrapper>

{#if existing !== undefined}
	<ConfirmDialog
		open={confirmTest}
		title="Confirm Test"
		content="Are you sure you want to test this pipeline? This will ignore delays and repeated actions and execute all actions you've listed including shutdown actions, it's recommend you save any changes if you have destructive actions. Only saved actions will be executed"
		onConfirm={() => {
			doTestMutation(existing.id);
			confirmTest = false;
		}}
		onCancel={() => (confirmTest = false)} />
{/if}

<CreateActionForm
	open={addAction}
	onSubmit={(action) => {
		addAction = false;
		actions.push(createLocalAction(action));
		actions = actions;
	}}
	onCancel={() => (addAction = false)} />

<EditActionForm
	open={editingAction !== null && editAction !== null}
	action={editingAction}
	onSubmit={(action) => {
		if (editAction !== null) {
			actions[editAction] = createLocalAction(action);
			actions = actions;
		}

		editAction = null;
	}}
	onCancel={() => (editAction = null)} />

{#if existing !== undefined}
	<DeletePipelineDialog
		open={confirmDelete}
		pipeline={existing}
		onClose={() => (confirmDelete = false)} />
{/if}

<style lang="scss">
	@use '$styles/palette.scss' as palette;

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
