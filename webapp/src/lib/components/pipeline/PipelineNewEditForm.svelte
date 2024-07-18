<script lang="ts">
	import {
		type EventPipeline,
		type Action,
		type UpdateEventPipeline,
		EventLevel,
		EVENT_TYPES,
		EVENT_TYPE_DATA,
		EventType,
		type CreateEventPipeline
	} from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { base } from '$app/paths';
	import ActionItem from '$lib/components/pipeline/ActionItem.svelte';
	import CreateActionForm from '$lib/components/pipeline/CreateActionForm.svelte';
	import EditActionForm from '$lib/components/pipeline/EditActionForm.svelte';
	import { Combobox, Label, Switch } from 'bits-ui';
	import InfoIcon from '~icons/solar/info-circle-bold-duotone';
	import WarningIcon from '~icons/solar/danger-triangle-bold-duotone';
	import ErrorIcon from '~icons/solar/bug-bold-duotone';
	import SuccessIcon from '~icons/solar/check-circle-bold-duotone';
	import { _ } from 'svelte-i18n';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';
	import Breadcrumbs from '../Breadcrumbs.svelte';
	import DeletePipelineDialog from './DeletePipelineDialog.svelte';
	import { Container } from '..';

	// Existing pipeline to edit if editing
	export let existing: EventPipeline | undefined = undefined;

	let confirmDelete = false;
	let addAction = false;
	let editAction: number | null = null;

	const client = useQueryClient();

	type UpdateData = { id: number; data: UpdateEventPipeline };

	type CreateData = { data: CreateEventPipeline };

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

	let eventType: EventType = EventType.ACFailure;
	let name: string = '';
	let cancellable: boolean = false;
	let actions: Action[] = [];

	$: {
		setDefaultState(existing);
	}

	$: editingAction = editAction === null ? null : actions[editAction];

	const removeAction = (index: number) => {
		actions.splice(index, 1);
		actions = actions;
	};

	const setDefaultState = (existing?: EventPipeline) => {
		if (existing) {
			eventType = existing.event;
			name = existing.name;
			cancellable = existing.cancellable;
			actions = [...existing.pipeline.actions];
		} else {
			eventType = EventType.ACFailure;
			name = '';
			cancellable = false;
			actions = [];
		}
	};

	$: values = EVENT_TYPES.map((eventType) => ({
		value: eventType,
		label: $_(`events.${eventType}.label`),
		description: $_(`events.${eventType}.description`)
	}));

	let inputValue = '';
	let touchedInput = false;

	$: filteredValues =
		inputValue && touchedInput
			? values.filter((value) => value.label.toLowerCase().includes(inputValue.toLowerCase()))
			: values;
</script>

<Container.Wrapper>
	<Breadcrumbs
		parts={[
			{ label: 'Event Pipelines', href: `${base}/pipelines` },
			existing !== undefined ? { label: existing.name } : { label: 'Create' }
		]}
	/>

	<Container.Root>
		<Container.Header>
			<h2 class="title">
				{#if existing !== undefined}
					Editing Pipeline <span class="pipeline-name">{existing.name}</span>
				{:else}
					Create Event Pipeline
				{/if}
			</h2>
		</Container.Header>

		<div class="settings">
			{#if existing === undefined}
				<div class="field">
					<h3 class="field__name">Event</h3>
					<div class="field__content">
						<Label.Root
							id="eventTypeLabel"
							for="eventType"
							class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
						>
							Choose an event this pipeline should run on
						</Label.Root>
						<Combobox.Root items={filteredValues} bind:inputValue bind:touchedInput>
							<Combobox.Input placeholder="Select an event">Test</Combobox.Input>
							<Combobox.Label id="eventType" />

							<Combobox.Content sideOffset={8} sameWidth={false} class="flex flex-col gap-4">
								{#each filteredValues as eventType}
									{@const typeData = EVENT_TYPE_DATA[eventType.value]}
									{#if typeData !== undefined}
										<Combobox.Item value={eventType.value}>
											<Combobox.ItemIndicator />
											<div class="event-item">
												<div class="event-item__icon">
													{#if typeData.level === EventLevel.Info}
														<span class="level level--info">
															<InfoIcon />
														</span>
													{:else if typeData.level === EventLevel.Success}
														<span class="level level--success">
															<SuccessIcon />
														</span>
													{:else if typeData.level === EventLevel.Warning}
														<span class="level level--warning">
															<WarningIcon />
														</span>
													{:else if typeData.level === EventLevel.Severe}
														<span class="level level--severe">
															<ErrorIcon />
														</span>
													{/if}
												</div>

												<div class="event-item__text">
													<p class="event-item__label">{eventType.label}</p>
													<p class="event-item__description">{eventType.description}</p>
												</div>
											</div>
										</Combobox.Item>
									{/if}
								{:else}
									<span> No results found </span>
								{/each}
							</Combobox.Content>
							<Combobox.Arrow />
							<Combobox.HiddenInput bind:value={eventType} />
						</Combobox.Root>
					</div>
				</div>
			{/if}

			<div class="field">
				<h3 class="field__name">Name</h3>
				<div class="field__content">
					<Label.Root
						id="eventTypeLabel"
						for="eventType"
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						Choose a name for this pipeline
					</Label.Root>
					<input class="input" id="name" type="text" required maxlength="100" bind:value={name} />
				</div>
			</div>
			<div class="field">
				<h3 class="field__name">Cancellable</h3>
				<div class="field__content">
					<Label.Root
						id="eventTypeLabel"
						for="eventType"
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						When cancelling is enabled if this task is running and the opposite of the event is
						received the current execution will be canceled
					</Label.Root>
					<Switch.Root
						checked={cancellable}
						onCheckedChange={(value) => {
							cancellable = value;
						}}
					>
						<Switch.Thumb />
					</Switch.Root>
				</div>
			</div>
		</div>

		<div class="content">
			<div class="items">
				{#each actions as action, index}
					<ActionItem
						{index}
						item={action}
						onEdit={() => (editAction = index)}
						onRemove={() => removeAction(index)}
					/>
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
									name,
									cancellable,
									pipeline: {
										actions
									}
								}
							});
						}}
					>
						Save
					</button>

					<button
						class="button button--secondary"
						on:click={() => {
							setDefaultState(existing);
							toast.info('Reverted changes.');
						}}
					>
						Reset
					</button>
					<button
						class="button button--secondary"
						on:click={() => {
							confirmDelete = true;
						}}
					>
						Delete
					</button>
				{:else}
					<button
						class="button"
						on:click={() => {
							$createPipelineMutation.mutate({
								data: { name, event: eventType, pipeline: { actions }, cancellable }
							});
						}}
					>
						Create
					</button>
				{/if}
				<a class="button button--secondary" href="{base}/pipelines">Back</a>
			</svelte:fragment>
		</Container.Footer>
	</Container.Root>
</Container.Wrapper>

{#if addAction}
	<CreateActionForm
		onSubmit={(action) => {
			addAction = false;
			actions.push(action);
			actions = actions;
		}}
		onCancel={() => (addAction = false)}
	/>
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
		onCancel={() => (editAction = null)}
	/>
{/if}

{#if confirmDelete && existing !== undefined}
	<DeletePipelineDialog pipeline={existing} onClose={() => (confirmDelete = false)} />
{/if}

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

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

	.title {
		font-size: 1.25rem;
		color: palette.$gray-800;
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

	.event-item {
		display: flex;
		gap: 1rem;
		align-items: center;
		padding: 0.5rem 1rem;

		&__text {
			display: flex;
			flex-flow: column;
			gap: 0.25rem;
		}

		&__label {
			font-weight: bold;
		}
		&__description {
			font-size: 0.9rem;
		}
	}

	.level {
		font-size: 1.25rem;
		line-height: 1;

		&--info {
			color: #34495e;
		}

		&--success {
			color: #30b455;
		}

		&--warning {
			color: #efaf13;
		}

		&--severe {
			color: #aa1109;
		}
	}
</style>
