<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { Combobox, Label, Switch } from 'bits-ui';
	import {
		EVENT_TYPE_DATA,
		EVENT_TYPES,
		EventLevel,
		type EventPipeline,
		EventType,
		type ActionPipeline,
		type CreateEventPipeline
	} from '$lib/api/types';
	import InfoIcon from '~icons/solar/info-circle-bold-duotone';
	import WarningIcon from '~icons/solar/danger-triangle-bold-duotone';
	import ErrorIcon from '~icons/solar/bug-bold-duotone';
	import SuccessIcon from '~icons/solar/check-circle-bold-duotone';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { HttpMethod, requestJson } from '$lib/api/utils';

	let eventType: EventType = EventType.ACFailure;
	let name: string = '';
	let cancellable: boolean = false;
	let pipeline: ActionPipeline = { actions: [] };

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

	const client = useQueryClient();

	// Mutation to update the player details
	const createPipelineMutation = createMutation({
		mutationFn: async () =>
			await requestJson<EventPipeline, CreateEventPipeline>({
				method: HttpMethod.POST,
				route: '/api/event-pipelines',
				body: { name, event: eventType, pipeline, cancellable }
			}),

		// Invalidate the current player details
		onSuccess: () => {
			// Redirect to pipelines list
			goto(`${base}/pipelines`);

			client.invalidateQueries({ queryKey: ['event-pipelines'] });
		}
	});
</script>

<div class="content">
	<form class="actions" on:submit|preventDefault={() => $createPipelineMutation.mutate()}>
		<div class="actions__header">
			<h2 class="actions__header__title">Create Event Pipeline</h2>
		</div>

		<div class="actions__content">
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
						<Combobox.Input placeholder="Select an event" />
						<Combobox.Label id="eventType" />

						<Combobox.Content>
							{#each filteredValues as eventType}
								{@const typeData = EVENT_TYPE_DATA[eventType.value]}
								{#if typeData !== undefined}
									<Combobox.Item value={eventType.value}>
										<Combobox.ItemIndicator />
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

										<div>
											<p>{eventType.label}</p>
											<p>{eventType.description}</p>
										</div>
									</Combobox.Item>
								{/if}
							{/each}
						</Combobox.Content>
						<Combobox.Arrow />
						<Combobox.HiddenInput bind:value={eventType} />
					</Combobox.Root>
				</div>
			</div>

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
					<input id="name" type="text" bind:value={name} />
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

		<div class="actions__header">
			<h2 class="actions__header__title">Actions</h2>
		</div>

		<div class="actions__content"></div>

		<div class="actions__footer">
			<button type="submit" class="button" disabled={$createPipelineMutation.isPending}
				>Create</button
			>
			<a class="button" href="/pipelines">Cancel</a>
		</div>
	</form>
</div>

<style lang="scss">
	@use '../../../lib/styles/palette.scss' as palette;

	.field {
		&__name {
			font-weight: bold;
			margin-bottom: 0.5rem;
			font-size: 1rem;
		}

		&__content {
		}
	}

	.actions {
		background-color: #fff;
		border: 0.1rem solid #dfe3e8;
		border-radius: 0.25rem;
		margin: 0 auto;
		max-width: 70rem;

		&__header {
			border-bottom: 0.1rem solid #dfe3e8;
			padding: 1rem;
			display: flex;
			justify-content: space-between;
			align-items: center;

			&__title {
				font-size: 1.25rem;
				color: palette.$gray-800;
			}
		}

		&__footer {
			border-top: 0.1rem solid #dfe3e8;
			padding: 1.5rem;
			display: flex;
			gap: 1rem;
		}
	}

	.actions__content {
		padding: 1rem;
		display: flex;
		flex-flow: column;
		gap: 0.5rem;
	}

	.empty {
		padding: 1rem;

		&__text {
			color: palette.$gray-800;
		}

		&__link {
			color: palette.$gray-700;
			font-weight: bold;
		}
	}

	.content {
		padding: 1rem;
	}

	.actions-create {
		border: none;
		padding: 0.5rem 0.75rem;
		font-size: 0.9rem;
		border-radius: 0.25rem;
		cursor: pointer;
		background-color: palette.$gray-700;
		color: white;
		text-decoration: none;
	}
</style>
