import { QueryClient } from '@tanstack/svelte-query';
import { browser } from '$app/environment';

// Base url segment
const BASE_URL = import.meta.env.VITE_SERVER_URL ?? `${window.location.origin}/api/`;

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			enabled: browser
		}
	}
});

// Constant enum for the different HTTP verbs
export const enum HttpMethod {
	GET = 'GET',
	POST = 'POST',
	PUT = 'PUT',
	DELETE = 'DELETE'
}

// Configuration object structure for requests
export interface RequestConfig<B> {
	// The http request method
	method: HttpMethod;
	// Route segment of the request URL
	route: string;
	// Body object for the request to encode as JSON
	body?: B;
	// Additional HTTP headers for the request
	headers?: Record<string, string>;
}

async function requestInner<B>(config: RequestConfig<B>): Promise<Response> {
	const init: RequestInit = { method: config.method };
	const headers: Record<string, string> = config.headers ?? {};

	// Assign the body value for non GET requests if the body is present
	if (config.method != HttpMethod.GET && config.body) {
		headers['Content-Type'] = 'application/json';
		init.body = JSON.stringify(config.body);
	}

	init.headers = headers;
	init.credentials = 'include';

	// Make the initial request
	let response: Response;
	try {
		const url = new URL(config.route, BASE_URL);
		response = await fetch(url, init);
	} catch (error) {
		console.error(error);
		throw new Error('Failed to connect', { cause: error });
	}

	return response;
}

export async function requestText<B = never>(config: RequestConfig<B>): Promise<string> {
	const response = await requestInner(config);
	/// Handle 2xx status codes
	if (response.ok) {
		// Handle invalid JSON responses
		try {
			return await response.text();
		} catch (error) {
			throw new Error('Invalid server response', {
				cause: error
			});
		}
	}

	// Handle non 200 status codes by taking the text response
	let text: string;
	try {
		text = await response.text();
	} catch (error) {
		throw new Error('Failed to get error response text', {
			cause: error
		});
	}

	throw new HttpResponseError(response.status, text);
}

/**
 * Creates a request that does not capture a response
 * and only cares that the response was a 2xx status code
 *
 * @param config Request configuration
 * @returns Promise for the request completion
 */
export async function requestStatus<B = never>(config: RequestConfig<B>): Promise<void> {
	const response = await requestInner(config);

	/// Handle 2xx status codes
	if (response.ok) {
		return;
	}

	// Handle non 200 status codes by taking the text response
	let text: string;
	try {
		text = await response.text();
	} catch (error) {
		throw new Error('Failed to get error response text', {
			cause: error
		});
	}

	throw new HttpResponseError(response.status, text);
}

export class HttpResponseError extends Error {
	statusCode: number;

	constructor(statusCode: number, message?: string, options?: ErrorOptions) {
		super(message, options);
		this.statusCode = statusCode;
	}
}

export async function requestJson<T, B = never>(config: RequestConfig<B>): Promise<T> {
	const response = await requestInner(config);

	if (response.ok) {
		// Handle invalid JSON responses
		try {
			return await response.json();
		} catch (error) {
			throw new Error('Invalid server response', {
				cause: error
			});
		}
	}

	// Handle non 200 status codes by taking the text response
	let text: string;
	try {
		text = await response.text();
	} catch (error) {
		throw new HttpResponseError(response.status, 'Failed to get error response text', {
			cause: error
		});
	}

	throw new HttpResponseError(response.status, text);
}
