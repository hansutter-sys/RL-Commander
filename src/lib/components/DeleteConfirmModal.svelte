<script lang="ts">
  import { showDeleteModal, activePaneState, activePane, refreshPane } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { deleteItems } from "../tauri";
  import { Trash2, AlertTriangle, X } from "lucide-svelte";

  $: itemsToDelete = Array.from($activePaneState.selectedPaths).map((p) => {
    const fileObj = $activePaneState.files.find((f) => f.path === p);
    return [p, fileObj ? fileObj.is_dir : false] as [string, boolean];
  });

  $: if (itemsToDelete.length === 0 && $activePaneState.files[$activePaneState.selectedIndex]) {
    const current = $activePaneState.files[$activePaneState.selectedIndex];
    if (current.name !== "..") {
      itemsToDelete = [[current.path, current.is_dir]];
    }
  }

  async function handleDelete() {
    if (itemsToDelete.length === 0) return;
    try {
      await deleteItems(itemsToDelete, $activePaneState.sftpConfig);
      refreshPane($activePane);
      showDeleteModal.set(false);
    } catch (e: any) {
      alert(`Fel vid radering:\n${e?.toString()}`);
    }
  }
</script>

{#if $showDeleteModal}
  <div class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div
      class="border rounded-xl w-full max-w-md shadow-2xl p-5 font-sans transition-colors
        {$settings.theme === 'light' ? 'bg-white border-rose-300 text-slate-800' : 'bg-slate-900 border-rose-900/60 text-slate-100'}"
    >
      <div class="flex items-center justify-between pb-3 border-b {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
        <div class="flex items-center gap-2 font-bold text-lg text-rose-500">
          <Trash2 class="w-5 h-5 text-rose-500" />
          <span>F8: Radera Objekten?</span>
        </div>
        <button
          on:click={() => showDeleteModal.set(false)}
          class="{$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="space-y-3 mt-4 font-mono text-xs">
        <div
          class="flex items-start gap-2 border p-3 rounded-lg
            {$settings.theme === 'light' ? 'bg-rose-50 border-rose-200 text-rose-800' : 'bg-rose-950/40 border-rose-800/40 text-rose-200'}"
        >
          <AlertTriangle class="w-5 h-5 text-rose-500 flex-shrink-0 mt-0.5" />
          <div>
            Är du säker på att du vill ta bort följande <span class="font-bold underline">{itemsToDelete.length}</span> objekt permanent?
          </div>
        </div>

        <div
          class="max-h-40 overflow-y-auto p-2 rounded border space-y-1 font-mono
            {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200 text-slate-700' : 'bg-slate-950 border-slate-800 text-slate-300'}"
        >
          {#each itemsToDelete as [path, is_dir]}
            <div class="truncate">
              <span class="text-rose-500 font-bold">{is_dir ? "[DIR]" : "[FILE]"}</span> {path}
            </div>
          {/each}
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
          <button
            on:click={() => showDeleteModal.set(false)}
            class="px-4 py-2 rounded font-semibold transition-colors
              {$settings.theme === 'light' ? 'bg-slate-100 hover:bg-slate-200 text-slate-700' : 'bg-slate-800 hover:bg-slate-700 text-slate-300'}"
          >
            Avbryt
          </button>
          <button
            on:click={handleDelete}
            class="px-4 py-2 rounded bg-rose-600 hover:bg-rose-500 text-white font-semibold flex items-center gap-1.5 shadow-lg shadow-rose-600/30"
          >
            <Trash2 class="w-4 h-4" />
            Ja, Radera
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
