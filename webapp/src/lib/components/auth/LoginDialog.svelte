<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { HttpMethod, requestText } from '$lib/api/utils';
	import type { LoginRequest } from '$lib/api/types';

	const client = useQueryClient();

	export let open = true;
	export let onClose: () => void = () => (open = false);

	const loginMutation = createMutation({
		mutationFn: async (password: string) =>
			await requestText<LoginRequest>({
				method: HttpMethod.POST,
				route: `/api/login`,
				body: { password }
			}),

		onSuccess: () => {
			client.invalidateQueries({ queryKey: ['login-state'] });
			onClose();
		}
	});

	let password = '';
</script>

<div class="container">
	<form
		on:submit|preventDefault={() => {
			$loginMutation.mutate(password);
		}}
	>
		<div class="dialog__header"><h3>{$_('login')}</h3></div>

		<div class="dialog__content">
			<p class="description">{$_('login_description')}</p>
			<div class="field">
				<label class="field__label" for="password">Password</label>
				<input class="field__input input" type="text" bind:value={password} required />
			</div>
		</div>
		<div class="dialog__footer">
			<div class="dialog__footer__actions">
				<button
					type="submit"
					class="button"
					disabled={password.length === 0 || $loginMutation.isPending}>Login</button
				>
				<div style="flex: auto;"></div>
				<slot name="actions" />
			</div>
		</div>
	</form>
</div>

<style lang="scss">
	@use '../../styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.container {
		margin: 2rem auto;
		width: 100%;
		max-width: 40rem;

		background-color: #fff;
		border: $border;
		border-radius: 0.25rem;

		max-height: 40rem;
		overflow: auto;

		z-index: 50;
	}

	.description {
		margin-bottom: 1rem;
		font-size: 0.9rem;
		color: palette.$gray-600;
	}

	.dialog__header {
		background-color: palette.$gray-200;
		padding: 1rem;
		border-bottom: $border;
		color: palette.$gray-700;
	}

	.dialog__content {
		max-height: 40rem;
		overflow: auto;
		padding: 1rem;
	}

	.dialog__footer {
		display: flex;
		padding: 1rem;

		justify-content: space-between;

		border-top: $border;
	}

	.dialog__footer__actions {
		display: flex;
		flex: auto;
		align-items: center;
		gap: 1rem;
	}
</style>
