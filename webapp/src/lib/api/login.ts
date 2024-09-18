import { createQuery } from '@tanstack/svelte-query';
import type { LoginState } from './types';
import { HttpMethod, requestJson } from './utils';

/**
 * Creates a query that will request the login
 * state from the backend
 */
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
