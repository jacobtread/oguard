<script lang="ts">
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import { Container } from '$lib/components';
	import { _ } from 'svelte-i18n';
</script>

<Container.Wrapper>
	<Breadcrumbs parts={[{ label: $_('pages.history') }]} />

	<Container.Root>
		<Container.Header title={$_('pages.history')} />

		<Container.Content>
			<Container.Section indent></Container.Section>
		</Container.Content>

		<Container.Footer></Container.Footer>
	</Container.Root>
</Container.Wrapper>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;
</style>
