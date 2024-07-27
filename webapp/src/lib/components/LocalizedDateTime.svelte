<script lang="ts">
	import dayjs from 'dayjs';

	export let value: string;
</script>

{dayjs(value).format('L LT')}
