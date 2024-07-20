<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { HttpMethod, requestText } from '$lib/api/utils';
	import type { LoginRequest } from '$lib/api/types';
	import { Container } from '..';

	export let open = true;
	export let onClose: () => void = () => (open = false);

	const client = useQueryClient();

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

	function onSubmitLogin() {
		$loginMutation.mutate(password);
	}
</script>

<Container.Wrapper maxWidth="xs">
	<Container.Root>
		<form on:submit|preventDefault={onSubmitLogin}>
			<Container.Header dark title={$_('login')}></Container.Header>

			<Container.Content>
				<Container.Section>
					<p class="description">{$_('login_description')}</p>
					<div class="field">
						<label class="field__label" for="password">Password</label>
						<input class="field__input input" type="text" bind:value={password} required />
					</div>
				</Container.Section>
			</Container.Content>

			<Container.Footer>
				<div class="dialog__footer__actions">
					<button
						type="submit"
						class="button"
						disabled={password.length === 0 || $loginMutation.isPending}>Login</button>
					<div style="flex: auto;"></div>
					<slot name="actions" />
				</div>
			</Container.Footer>
		</form>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.description {
		margin-bottom: 1rem;
		font-size: 0.9rem;
		color: palette.$gray-600;
	}

	.dialog__footer__actions {
		display: flex;
		flex: auto;
		align-items: center;
		gap: 1rem;
	}
</style>
