import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import path from 'node:path';

import { dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess({
		style: {
			css: {
				preprocessorOptions: {
					scss: {
						importer: [
							(url) => {
								if (url.startsWith('$lib')) {
									return {
										file: url.replace(/^\$lib/, path.join(__dirname, 'src', 'lib'))
									};
								}
								return url;
							}
						]
					}
				}
			}
		}
	}),

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter({
			fallback: 'index.html'
		}),
		alias: {
			$: path.resolve('src'),
			$styles: path.resolve('src/lib/styles'),
			$components: path.resolve('src/lib/components')
		}
	}
};

export default config;
