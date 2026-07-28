<script lang="ts">
	import { page } from '$app/stores';
	import { schemaLoader, DynamicPage } from '@tauri-front/shared';

	$: currentPage = $page.url.pathname === '/' 
		? schemaLoader.getPageByRoute('/dashboard')
		: schemaLoader.getPageByRoute($page.url.pathname);
</script>

{#if currentPage}
	<DynamicPage schema={currentPage} />
{:else}
	<div class="p-8 text-center text-gray-500 dark:text-gray-400">
		<p>Page not found: {$page.url.pathname}</p>
	</div>
{/if}
