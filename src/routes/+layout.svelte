<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { schemaLoader, ThemeService } from '@tauri-front/shared';

	let currentPath = '/';
	let schemaLoaded = false;
	let schemaError = null;

	page.subscribe((p) => {
		currentPath = p.url.pathname;
	});

	onMount(async () => {
		ThemeService.init();
		try {
			await schemaLoader.loadFromUrl('/schemas/cleanuxschemas.json');
			schemaLoaded = true;
		} catch (e) {
			schemaError = e.message;
			console.error('Failed to load schema:', e);
		}
	});
</script>

{#if schemaError}
	<div class="min-h-screen bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100 flex items-center justify-center">
		<div class="bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-600 text-red-700 dark:text-red-300 px-6 py-4 rounded-lg">
			<p class="font-bold">Schema Load Error</p>
			<p>{schemaError}</p>
		</div>
	</div>
{:else if !schemaLoaded}
	<div class="min-h-screen bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100 flex items-center justify-center">
		<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-gray-100"></div>
	</div>
{:else}
	<slot />
{/if}
