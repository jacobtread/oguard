import { createQuery } from '@tanstack/svelte-query';
import type { LoginState } from './types';
import { HttpMethod, requestJson } from './utils';

export function createLoginStateQuery() {
	return createQuery<LoginState>({
		queryKey: ['login-state'],
		queryFn: async () =>
			await requestJson<LoginState>({
				method: HttpMethod.GET,
				route: '/api/login-state'
			})
	});
}
