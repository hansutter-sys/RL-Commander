<script lang="ts">
  import { showViewerModal, viewerFilePath, activePaneState } from "../stores/commander";
  import { readFileText } from "../tauri";
  import { FileText, X, Copy } from "lucide-svelte";
  import { onMount } from "svelte";

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
    <div class="bg-slate-900 border border-slate-700 rounded-xl w-full max-w-4xl h-[80vh] flex flex-col shadow-2xl overflow-hidden">
      <!-- Header -->
      <div class="bg-slate-950 px-4 py-3 border-b border-slate-800 flex items-center justify-between">
        <div class="flex items-center gap-2 font-mono text-sm text-sky-400 font-bold truncate">
          <FileText class="w-4 h-4 text-sky-400" />
          <span class="truncate">F3 Viewer: {$viewerFilePath}</span>
        </div>
        <button on:click={() => showViewerModal.set(false)} class="text-slate-400 hover:text-white">
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- File Content View -->
      <div class="flex-1 overflow-auto p-4 bg-slate-950 text-slate-200 font-mono text-xs leading-relaxed whitespace-pre-wrap select-text border-b border-slate-800">
        {#if loading}
          <div class="flex items-center justify-center h-full text-slate-500 italic">
            Laddar...
          </div>
        {:else}
          {content}
        {/if}
      </div>

      <!-- Footer -->
      <div class="bg-slate-950 px-4 py-2 flex items-center justify-between text-xs font-mono text-slate-400">
        <span>Tryck ESC eller klicka stäng för att gå tillbaka</span>
        <button
          on:click={() => navigator.clipboard.writeText(content)}
          class="px-3 py-1 rounded bg-slate-800 hover:bg-slate-700 text-slate-200 flex items-center gap-1 border border-slate-700"
        >
          <Copy class="w-3 h-3" />
          Kopiera text
        </button>
      </div>
    </div>
  </div>
{/if}
