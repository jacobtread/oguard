import { watch, Context } from 'runed';
import IntlMessageFormat from 'intl-messageformat';

interface I18nContext {
	loading: boolean;
	error: Error | null;

	locale: string;
	f: (key: string, options?: FormatOptions) => string;
}

interface FormatOptions {
	values: Partial<Record<string, unknown>>;
}

export const i18nContext = new Context<I18nContext>('I18nContext');

type LocaleData = Partial<Record<string, string>>;

// Loaders for the various locales
const loaders: Record<string, () => Promise<LocaleData>> = {
	en: () => import('./locales/en.json').then((value) => flattenLocale(value.default))
};

type LocaleEntry = string | { [key: string]: LocaleEntry };

function flattenLocale(input: Record<string, LocaleEntry>) {
	const output: Record<string, string> = {};

	const flattenObject = (
		input: Record<string, LocaleEntry>,
		output: Record<string, string>,
		parentKey?: string
	) => {
		for (const key of Object.keys(input)) {
			const nestedKey = parentKey === undefined ? key : `${parentKey}.${key}`;
			const value = input[key];

			if (typeof value === 'string') {
				output[nestedKey] = value;
			} else {
				flattenObject(value, output, nestedKey);
			}
		}
	};

	flattenObject(input, output);

	return output;
}

export function createI18n(): I18nContext {
	let locale: string = $state('en');

	let loading: boolean = $state(true);
	let error: Error | null = $state(null);

	let localeData: LocaleData = $state({});
	let cancel: VoidFunction | undefined;

	const loadLocale = (locale: string) => {
		let cancelled = false;

		const loader = loaders[locale];
		const promise = loader();

		loading = true;
		error = null;

		promise
			.then((data) => {
				if (cancelled) return;

				cancel = undefined;
				loading = false;
				localeData = data;
			})
			.catch((err) => {
				if (cancelled) return;

				cancel = undefined;
				loading = false;

				if (err instanceof Error) {
					error = err;
				} else {
					error = new Error('Unknown error occurred');
				}
			});

		return () => {
			cancelled = true;
		};
	};

	// Load locale based on current value
	watch(
		() => locale,
		(locale) => {
			localeData = {};
			loading = true;

			// Cancel any current loading
			if (cancel) {
				cancel();
			}

			cancel = loadLocale(locale);
		}
	);

	return {
		get loading() {
			return loading;
		},

		get error() {
			return error;
		},

		get locale() {
			return locale;
		},

		set locale(value: string) {
			if (locale !== value) {
				locale = value;
			}
		},

		f: (key, options) => {
			// Currently loading the locale data
			if (loading) {
				return key;
			}

			const value = localeData[key];

			// No translation available
			if (value === undefined) {
				return key;
			}

			const message = new IntlMessageFormat(value, locale);
			return message.format(options?.values) as string;
		}
	};
}
