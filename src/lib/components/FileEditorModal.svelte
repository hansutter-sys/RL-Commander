<script lang="ts">
  import { showEditorModal, editorFilePath, activePaneState, refreshPane, activePane } from "../stores/commander";
  import { readFileText, writeFileText } from "../tauri";
  import { Edit3, X, Save, CheckCircle, AlertCircle, RefreshCw } from "lucide-svelte";

  let content = "";
  let initialContent = "";
  let loading = false;
  let saving = false;
  let statusMessage: { type: "success" | "error"; text: string } | null = null;

  $: if ($showEditorModal && $editorFilePath) {
    loadFile();
  }

  $: isDirty = content !== initialContent;

  async function loadFile() {
    loading = true;
    statusMessage = null;
    try {
      content = await readFileText($editorFilePath, 500000, $activePaneState.sftpConfig);
      initialContent = content;
    } catch (e: any) {
      content = "";
      initialContent = "";
      statusMessage = { type: "error", text: `Fel vid läsning: ${e?.toString()}` };
    } finally {
      loading = false;
    }
  }

  async function saveFile() {
    if (!$editorFilePath || saving) return;
    saving = true;
    statusMessage = null;
    try {
      await writeFileText($editorFilePath, content, $activePaneState.sftpConfig);
      initialContent = content;
      statusMessage = { type: "success", text: "Filen sparades framgångsrikt!" };
      refreshPane($activePane);
      setTimeout(() => {
        if (statusMessage?.type === "success") statusMessage = null;
      }, 3000);
    } catch (e: any) {
      statusMessage = { type: "error", text: `Sparfel: ${e?.toString()}` };
    } finally {
      saving = false;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      saveFile();
    }
  }

  function closeModal() {
    if (isDirty) {
      if (confirm("Du har osparade ändringar. Vill du stänga utan att spara?")) {
        showEditorModal.set(false);
      }
    } else {
      showEditorModal.set(false);
    }
  }
</script>

{#if $showEditorModal}
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <div class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-6" on:keydown={handleKeyDown}>
    <div class="bg-slate-900 border border-slate-700 rounded-xl w-full max-w-5xl h-[85vh] flex flex-col shadow-2xl overflow-hidden">
      <!-- Header -->
      <div class="bg-slate-950 px-4 py-3 border-b border-slate-800 flex items-center justify-between">
        <div class="flex items-center gap-2 font-mono text-sm text-amber-400 font-bold truncate">
          <Edit3 class="w-4 h-4 text-amber-400" />
          <span class="truncate">F4 Redigerare: {$editorFilePath}</span>
          {#if isDirty}
            <span class="bg-amber-950/80 text-amber-300 border border-amber-800/80 text-[10px] px-2 py-0.5 rounded font-mono animate-pulse">
              [Ändrad *]
            </span>
          {/if}
        </div>
        <div class="flex items-center gap-2">
          <button
            on:click={saveFile}
            disabled={saving || !isDirty}
            class="px-3 py-1.5 rounded-md text-xs font-mono font-semibold flex items-center gap-1.5 transition-all shadow-sm
              {isDirty ? 'bg-emerald-600 hover:bg-emerald-500 text-white cursor-pointer' : 'bg-slate-800 text-slate-500 cursor-not-allowed border border-slate-700'}"
          >
            {#if saving}
              <RefreshCw class="w-3.5 h-3.5 animate-spin" />
            {:else}
              <Save class="w-3.5 h-3.5" />
            {/if}
            <span>Spara (Ctrl+S)</span>
          </button>
          <button on:click={closeModal} class="text-slate-400 hover:text-white p-1 rounded hover:bg-slate-800">
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Status notification bar -->
      {#if statusMessage}
        <div class="px-4 py-2 border-b text-xs font-mono flex items-center gap-2
          {statusMessage.type === 'success' ? 'bg-emerald-950/80 border-emerald-800 text-emerald-300' : 'bg-rose-950/80 border-rose-800 text-rose-300'}">
          {#if statusMessage.type === "success"}
            <CheckCircle class="w-4 h-4 text-emerald-400" />
          {:else}
            <AlertCircle class="w-4 h-4 text-rose-400" />
          {/if}
          <span>{statusMessage.text}</span>
        </div>
      {/if}

      <!-- Editor TextArea Area -->
      <div class="flex-1 overflow-hidden bg-slate-950 p-4 font-mono text-xs flex">
        {#if loading}
          <div class="flex-1 flex items-center justify-center text-slate-500 italic">
            <RefreshCw class="w-5 h-5 animate-spin mr-2 text-amber-500" />
            Laddar filinnehåll...
          </div>
        {:else}
          <textarea
            bind:value={content}
            spellcheck="false"
            class="w-full h-full bg-slate-950 text-slate-100 font-mono text-xs p-3 leading-relaxed outline-none resize-none border border-slate-800 focus:border-amber-500/60 rounded-md selection:bg-amber-900/60"
            placeholder="Skriv text här..."
          ></textarea>
        {/if}
      </div>

      <!-- Footer -->
      <div class="bg-slate-950 px-4 py-2 flex items-center justify-between text-xs font-mono text-slate-400 border-t border-slate-800">
        <span>Rad: {content.split("\n").length} rader | Tecken: {content.length}</span>
        <span>Tryck Ctrl+S för att spara | Esc för att stänga</span>
      </div>
    </div>
  </div>
{/if}
