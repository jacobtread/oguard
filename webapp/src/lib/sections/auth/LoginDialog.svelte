<script lang="ts">
	import { t } from 'svelte-i18n';

	import Container from '$lib/components/container';
	import { createLoginMutation } from '$/lib/api/login';

	export let open = true;
	export let onClose: () => void = () => (open = false);

	const loginMutation = createLoginMutation();

	let password = '';

	const onSubmitLogin = async () => {
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
		<form on:submit|preventDefault={onSubmitLogin}>
			<Container.Header dark title={$t('login')}></Container.Header>

			<Container.Content>
				<Container.Section>
					<p class="description">{$t('login_description')}</p>
					<div class="field">
						<label class="field__label" for="password">{$t('password')}</label>
						<input class="field__input input" type="password" bind:value={password} required />
					</div>
				</Container.Section>
			</Container.Content>

			<Container.Footer>
				<div class="dialog__footer__actions">
					<button
						type="submit"
						class="button"
						disabled={password.length === 0 || loginMutation.isPending}>{$t('login')}</button>
					<div style="flex: auto;"></div>
					<slot name="actions" />
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
