<script lang="ts">
	import type { EventPipeline } from '$lib/api/types';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import { createQuery } from '@tanstack/svelte-query';
	import dayjs from 'dayjs';
	import { _ } from 'svelte-i18n';
	import SolarBoxBoldDuotone from '~icons/solar/box-bold-duotone';
	import { Switch } from 'bits-ui';
	import EditIcon from '~icons/solar/pen-bold';

	const eventPipelinesQuery = createQuery<EventPipeline[]>({
		queryKey: ['event-pipelines'],
		queryFn: async () =>
			await requestJson<EventPipeline[]>({
				method: HttpMethod.GET,
				route: '/api/event-pipelines'
			}),

		// Refetch the data every second
		refetchInterval: 3000
	});
</script>

<div class="content">
	<div class="actions">
		<div class="actions__header">
			<h2 class="actions__header__title">Actions</h2>

			<div class="action__buttons">
				<a class="actions-create" href="/pipelines/create">Create</a>
			</div>
		</div>
		{#if $eventPipelinesQuery.isPending}
			Loading...
		{/if}
		{#if $eventPipelinesQuery.error}
			An error has occurred:
			{$eventPipelinesQuery.error.message}
		{/if}
		{#if $eventPipelinesQuery.isSuccess}
			{#if $eventPipelinesQuery.data.length === 0}
				<div class="empty">
					<p class="empty__text">
						You don't have any event pipelines press
						<a class="empty__link" href="/pipelines/create"> Create </a>
						to create a new one
					</p>
				</div>
			{/if}
			{#each $eventPipelinesQuery.data as row}
				<div class="action">
					<div class="action__icon"><SolarBoxBoldDuotone /></div>
					<div class="action__text">
						<p class="action__name">{row.name}</p>
						<div class="action__timestamps">
							{#if row.modified_at !== null}
								<p class="action__timestamp">
									Last Modified <span>{dayjs(row.modified_at).format('L LT')}</span>
								</p>
							{/if}
							{#if row.last_executed_at !== null}
								<p class="action__timestamp">
									Last executed <span>{dayjs(row.last_executed_at).format('L LT')}</span>
								</p>
							{/if}
						</div>
					</div>
					<div>
						<Switch.Root
							checked={row.enabled}
							onCheckedChange={() => {
								// TODO: API CALL TO UPDATE ENABLED STATE
							}}
						>
							<Switch.Thumb />
						</Switch.Root>
					</div>

					<div class="action__menu">
						<a href="/pipelines/{row.id}"><EditIcon /></a>
					</div>
				</div>
			{/each}
		{/if}
		<div class="actions__footer"></div>
	</div>
</div>

<style lang="scss">
	@use '../../lib/styles/palette.scss' as palette;

	.action {
		display: flex;
		gap: 0.5rem;
		padding: 1rem;
		align-items: center;

		&__icon {
			font-size: 2rem;
			display: flex;
			align-items: center;
			justify-content: center;
		}

		&__text {
			flex: auto;
		}

		&__name {
			font-weight: bold;
			color: palette.$gray-800;
			margin-bottom: 0.25rem;
		}

		&__menu {
			justify-self: flex-end;
		}

		&__timestamp {
			font-size: 0.9rem;
			color: palette.$gray-600;

			> span {
				background-color: palette.$gray-200;
				padding: 0.25rem 0.5rem;
				margin-left: 0.25rem;
				border-radius: 0.25rem;
			}
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
			justify-content: space-between;
		}
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
