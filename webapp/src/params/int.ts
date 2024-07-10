import type { ParamMatcher } from "@sveltejs/kit";

// Param matching for integer values
export const match: ParamMatcher = (param: string) => {
    return /^\d+$/.test(param);
}

