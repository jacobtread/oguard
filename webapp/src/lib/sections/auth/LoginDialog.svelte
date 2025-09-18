<script lang="ts">
	import Container from '$lib/components/container';
	import { createLoginMutation } from '$lib/api/login';
	import { i18nContext } from '$lib/i18n/i18n.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		open?: boolean;
		onClose?: VoidFunction;
		actions?: Snippet;
	}

	let { open = $bindable(true), onClose = () => (open = false), actions }: Props = $props();

	const i18n = i18nContext.get();

	const loginMutation = createLoginMutation();

	let password = $state('');

	const onSubmitLogin = async (event: SubmitEvent) => {
		event.preventDefault();

		try {
			await loginMutation.mutateAsync(password);
		} catch (err) {
			console.error('login failed', err);
		} finally {
			onClose();
		}
	};
</script>

<Container.Wrapper maxWidth="xs">
	<Container.Root>
		<form onsubmit={onSubmitLogin}>
			<Container.Header dark title={i18n.f('login')}></Container.Header>

			<Container.Content>
				<Container.Section>
					<p class="description">{i18n.f('login_description')}</p>
					<div class="field">
						<label class="field__label" for="password">{i18n.f('password')}</label>
						<input class="field__input input" type="password" bind:value={password} required />
					</div>
				</Container.Section>
			</Container.Content>

			<Container.Footer>
				<div class="dialog__footer__actions">
					<button
						type="submit"
						class="button"
						disabled={password.length === 0 || loginMutation.isPending}>{i18n.f('login')}</button>
					<div style="flex: auto;"></div>

					{@render actions?.()}
				</div>
			</Container.Footer>
		</form>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

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
