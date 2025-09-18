<script lang="ts">
	import dayjs from 'dayjs';

	interface Props {
		value: string;
	}

	const { value }: Props = $props();
</script>

{dayjs(value).format('L LT')}
