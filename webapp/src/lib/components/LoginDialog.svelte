<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { fly } from 'svelte/transition';
	import { Dialog } from 'bits-ui';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { HttpMethod, requestText } from '$lib/api/utils';
	import type { LoginRequest } from '$lib/api/types';

	const client = useQueryClient();

	export let open = true;
	export let onClose: () => void = () => (open = false);

	// Mutation to update the player details
	const loginMutation = createMutation({
		mutationFn: async (password: string) =>
			await requestText<LoginRequest>({
				method: HttpMethod.POST,
				route: `/api/login`,
				body: { password }
			}),

		// Invalidate the current player details
		onSuccess: () => {
			client.invalidateQueries({ queryKey: ['login-state'] });
			onClose();
		}
	});

	let password = '';
</script>

<Dialog.Root {open} closeOnEscape={false} closeOnOutsideClick={false}>
	<Dialog.Portal>
		<Dialog.Overlay transition={fly} transitionConfig={{ duration: 300, y: -10 }} />
		<Dialog.Content transition={fly} transitionConfig={{ duration: 300, y: -10 }}>
			<form
				on:submit|preventDefault={() => {
					$loginMutation.mutate(password);
				}}
			>
				<div class="dialog__header"><h3>{$_('login')}</h3></div>

				<div class="dialog__content">
					<input type="text" bind:value={password} />
				</div>
				<div class="dialog__footer">
					<div class="dialog__footer__actions">
						<button
							type="submit"
							class="button"
							disabled={password.length === 0 || $loginMutation.isPending}>Login</button
						>
					</div>
				</div>
			</form>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

<style lang="scss">
	@use '../styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.dialog__header {
		background-color: palette.$gray-200;
		padding: 1rem;
		border-bottom: $border;
		color: palette.$gray-700;
	}

	.dialog__content {
		max-height: 40rem;
		overflow: auto;
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
