# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/main/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm create svelte@latest

# create a new project in my-app
npm create svelte@latest my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

Libraries good for the svelte v5 upgrade:
- [-] svelte-sonner
- [x] svelte-i18n
- [x] svelte-dnd-action
- [-] svelte-headless-table (They aren't going to upgrade. Migrate to tanstack table?)
- [x] date-picker-svelte
- [x] @tanstack/svelte-query-devtools
- [x] @tanstack/svelte-query
- [-] @carbon/charts-svelte (Maybe not yet?)
- [-] bits-ui (Yes? peer dep on 5.0.0-next) 