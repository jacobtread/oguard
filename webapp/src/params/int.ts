import type { ParamMatcher } from '@sveltejs/kit';

// Param matching for integer values
export const match: ParamMatcher = (param: string): param is string => {
	return /^\d+$/.test(param);
};
