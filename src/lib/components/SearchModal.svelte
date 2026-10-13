<script lang="ts">
  import { showSearchModal, activePaneState, activePane, navigatePane } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { searchFiles } from "../tauri";
  import type { FileItem } from "../types";
  import { Search, X, Folder, FileText, ArrowRight, RefreshCw } from "lucide-svelte";

  let query = "";
  let results: FileItem[] = [];
  let searching = false;
  let hasSearched = false;

  async function handleSearch() {
    if (!query.trim()) return;
    searching = true;
    hasSearched = true;
    try {
      results = await searchFiles($activePaneState.currentPath, query, $activePaneState.sftpConfig);
    } catch (err: any) {
      console.error("Search error:", err);
      results = [];
    } finally {
      searching = false;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      handleSearch();
    }
  }

  function jumpToItem(item: FileItem) {
    if (item.is_dir) {
      navigatePane($activePane, item.path);
    } else {
      const parentDir = item.path.substring(0, item.path.lastIndexOf("/"));
      if (parentDir) {
        navigatePane($activePane, parentDir);
      }
    }
    showSearchModal.set(false);
  }
</script>

{#if $showSearchModal}
  <div class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-6">
    <div
      class="border rounded-xl w-full max-w-3xl h-[75vh] flex flex-col shadow-2xl overflow-hidden transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <!-- Header -->
      <div
        class="px-4 py-3 border-b flex items-center justify-between transition-colors
          {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950 border-slate-800'}"
      >
        <div class="flex items-center gap-2 font-mono text-sm text-purple-500 font-bold">
          <Search class="w-4 h-4 text-purple-500" />
          <span>Sök Filer (Alt+F7) — Sökstig: {$activePaneState.currentPath}</span>
        </div>
        <button
          on:click={() => showSearchModal.set(false)}
          class="{$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Search Input Bar -->
      <div
        class="p-4 border-b flex gap-2 transition-colors
          {$settings.theme === 'light' ? 'bg-slate-50/50 border-slate-200' : 'bg-slate-900 border-slate-800'}"
      >
        <div class="relative flex-1">
          <input
            type="text"
            bind:value={query}
            on:keydown={handleKeyDown}
            placeholder="Skriv sökord (t.ex. *.rs, config, main)..."
            class="w-full border px-3 py-2 rounded font-mono text-xs outline-none pl-9 focus:border-purple-500
              {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
          />
          <Search class="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
        </div>
        <button
          on:click={handleSearch}
          disabled={searching}
          class="bg-purple-600 hover:bg-purple-500 text-white font-mono text-xs font-semibold px-4 py-2 rounded flex items-center gap-1.5 transition-all shadow-md shadow-purple-600/20"
        >
          {#if searching}
            <RefreshCw class="w-3.5 h-3.5 animate-spin" />
            <span>Söker...</span>
          {:else}
            <span>Sök</span>
          {/if}
        </button>
      </div>

      <!-- Results View -->
      <div
        class="flex-1 overflow-y-auto p-4 font-mono text-xs transition-colors
          {$settings.theme === 'light' ? 'bg-white text-slate-800' : 'bg-slate-950 text-slate-200'}"
      >
        {#if searching}
          <div class="flex items-center justify-center h-full text-slate-400 italic gap-2">
            <RefreshCw class="w-5 h-5 animate-spin text-purple-500" />
            Genomsöker undermappar...
          </div>
        {:else if hasSearched && results.length === 0}
          <div class="flex items-center justify-center h-full text-slate-400 italic">
            Inga filer hittades som matchar "{query}".
          </div>
        {:else if results.length > 0}
          <div class="space-y-1">
            {#each results as item}
              <button
                on:click={() => jumpToItem(item)}
                class="w-full text-left p-2 rounded border flex items-center justify-between group transition-all
                  {$settings.theme === 'light'
                    ? 'bg-slate-50 hover:bg-purple-50 border-slate-200 hover:border-purple-300 text-slate-800'
                    : 'bg-slate-900/60 hover:bg-purple-950/40 border-slate-800 hover:border-purple-800 text-slate-200'}"
              >
                <div class="flex items-center gap-2 truncate">
                  {#if item.is_dir}
                    <Folder class="w-4 h-4 text-sky-500 flex-shrink-0" />
                  {:else}
                    <FileText class="w-4 h-4 text-purple-500 flex-shrink-0" />
                  {/if}
                  <span class="font-bold {$settings.theme === 'light' ? 'text-slate-900 group-hover:text-purple-700' : 'text-slate-200 group-hover:text-purple-300'}">
                    {item.name}
                  </span>
                  <span class="text-[11px] text-slate-400 truncate">{item.path}</span>
                </div>
                <ArrowRight class="w-4 h-4 text-slate-400 group-hover:text-purple-500 flex-shrink-0 opacity-0 group-hover:opacity-100 transition-opacity" />
              </button>
            {/each}
          </div>
        {:else}
          <div class="flex flex-col items-center justify-center h-full text-slate-400 text-center space-y-2">
            <Search class="w-10 h-10 opacity-40" />
            <p>Ange ett sökmönster ovan och tryck Enter för att påbörja sökning.</p>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="px-4 py-2 border-t text-xs font-mono flex justify-between transition-colors
          {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200 text-slate-600' : 'bg-slate-950 border-slate-800 text-slate-400'}"
      >
        <span>Träffar: {results.length}</span>
        <span>Klicka på ett resultat för att navigera direkt dit</span>
      </div>
    </div>
  </div>
{/if}
