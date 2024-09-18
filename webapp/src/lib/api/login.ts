import { createMutation, createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
import type { LoginRequest, LoginState } from './types';
import { HttpMethod, queryClient, requestJson, requestText } from './utils';

export const LOGIN_STATE_KEY = 'login-state';

/**
 * Creates a query that will request the login
 * state from the backend
 */
export function createLoginStateQuery(): CreateQueryResult<LoginState, Error> {
	return createQuery<LoginState>({
		queryKey: [LOGIN_STATE_KEY],
		queryFn: loginState
	});
}

/**
 * Creates a request to get the current login state
 *
 * @returns The current login state
 */
export function loginState(): Promise<LoginState> {
	return requestJson<LoginState>({
		method: HttpMethod.GET,
		route: '/api/login-state'
	});
}

/**
 * Creates a request to login using the provided
 * password
 *
 * @param password The password to login with
 */
export function loginRequest(password: string): Promise<string> {
	return requestText<LoginRequest>({
		method: HttpMethod.POST,
		route: `/api/login`,
		body: { password }
	});
}

/**
 * Creates a request to logout
 */
export function logoutRequest(): Promise<string> {
	return requestText({
		method: HttpMethod.POST,
		route: `/api/logout`
	});
}

export function createLoginMutation() {
	return createMutation<unknown, Error, string>({
		mutationFn: (password) => loginRequest(password),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: [LOGIN_STATE_KEY] });
		}
	});
}

export function createLogoutMutation() {
	return createMutation({
		mutationFn: logoutRequest,
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: [LOGIN_STATE_KEY] });
		}
	});
}
