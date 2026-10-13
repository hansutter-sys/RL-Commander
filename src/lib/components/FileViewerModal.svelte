<script lang="ts">
  import { showViewerModal, viewerFilePath, activePaneState } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { readFileText } from "../tauri";
  import { FileText, X, Copy } from "lucide-svelte";

  let content = "Laddar filinnehåll...";
  let loading = false;

  $: if ($showViewerModal && $viewerFilePath) {
    loadFile();
  }

  async function loadFile() {
    loading = true;
    try {
      content = await readFileText($viewerFilePath, 200000, $activePaneState.sftpConfig);
    } catch (e: any) {
      content = `Fel vid läsning av fil:\n${e?.toString()}`;
    } finally {
      loading = false;
    }
  }
</script>

{#if $showViewerModal}
  <div class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-6">
    <div
      class="border rounded-xl w-full max-w-4xl h-[80vh] flex flex-col shadow-2xl overflow-hidden transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <!-- Header -->
      <div
        class="px-4 py-3 border-b flex items-center justify-between transition-colors
          {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950 border-slate-800'}"
      >
        <div class="flex items-center gap-2 font-mono text-sm text-sky-500 font-bold truncate">
          <FileText class="w-4 h-4 text-sky-500" />
          <span class="truncate">F3 Viewer: {$viewerFilePath}</span>
        </div>
        <button
          on:click={() => showViewerModal.set(false)}
          class="{$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- File Content View -->
      <div
        class="flex-1 overflow-auto p-4 font-mono text-xs leading-relaxed whitespace-pre-wrap select-text border-b transition-colors
          {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200 text-slate-800' : 'bg-slate-950 border-slate-800 text-slate-200'}"
      >
        {#if loading}
          <div class="flex items-center justify-center h-full text-slate-400 italic">
            Laddar...
          </div>
        {:else}
          {content}
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="px-4 py-2 flex items-center justify-between text-xs font-mono transition-colors
          {$settings.theme === 'light' ? 'bg-slate-50 text-slate-600' : 'bg-slate-950 text-slate-400'}"
      >
        <span>Tryck ESC eller klicka stäng för att gå tillbaka</span>
        <button
          on:click={() => navigator.clipboard.writeText(content)}
          class="px-3 py-1 rounded flex items-center gap-1 border transition-colors
            {$settings.theme === 'light'
              ? 'bg-white hover:bg-slate-100 text-slate-700 border-slate-300'
              : 'bg-slate-800 hover:bg-slate-700 text-slate-200 border-slate-700'}"
        >
          <Copy class="w-3 h-3" />
          Kopiera text
        </button>
      </div>
    </div>
  </div>
{/if}
