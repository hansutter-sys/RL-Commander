<script lang="ts">
  import { showSettingsModal } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { Settings, X, HardDrive, Eye, Trash2, Folder, RotateCcw, Check, Monitor } from "lucide-svelte";

  function handleReset() {
    if (confirm("Är du säker på att du vill återställa alla inställningar till standard?")) {
      settings.reset();
    }
  }

  function handleClose() {
    showSettingsModal.set(false);
  }
</script>

{#if $showSettingsModal}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700 rounded-xl w-full max-w-lg shadow-2xl p-5 text-slate-100 font-sans flex flex-col max-h-[90vh]">
      <!-- Header -->
      <div class="flex items-center justify-between pb-3 border-b border-slate-800 flex-shrink-0">
        <div class="flex items-center gap-2 font-bold text-lg text-blue-400">
          <Settings class="w-5 h-5 text-blue-400" />
          <span>Inställningar</span>
        </div>
        <button on:click={handleClose} class="text-slate-400 hover:text-white p-1 rounded hover:bg-slate-800 transition-colors">
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Settings Content -->
      <div class="space-y-5 my-4 overflow-y-auto pr-1 flex-1 font-sans text-sm">
        <!-- Layout & Display Section -->
        <div class="space-y-3">
          <h3 class="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider flex items-center gap-1.5">
            <Monitor class="w-4 h-4 text-indigo-400" />
            Visning & Panelvy
          </h3>

          <div class="bg-slate-950/70 border border-slate-800 rounded-lg p-3 space-y-3">
            <!-- Toggle showDiskSpace -->
            <label class="flex items-center justify-between cursor-pointer group">
              <div class="space-y-0.5">
                <span class="font-medium text-slate-200 group-hover:text-blue-400 transition-colors">Visa diskutrymme</span>
                <p class="text-xs text-slate-400">Visar disk-användning, ledigt utrymme och procentbar under enhetsväljaren</p>
              </div>
              <input
                type="checkbox"
                checked={$settings.showDiskSpace}
                on:change={(e) => settings.update((s) => ({ ...s, showDiskSpace: e.currentTarget.checked }))}
                class="w-4 h-4 accent-blue-500 rounded border-slate-700 bg-slate-900 cursor-pointer"
              />
            </label>

            <div class="h-px bg-slate-800/60"></div>

            <!-- Toggle showDriveButtons -->
            <label class="flex items-center justify-between cursor-pointer group">
              <div class="space-y-0.5">
                <span class="font-medium text-slate-200 group-hover:text-blue-400 transition-colors">Visa snabbknappar för diskar</span>
                <p class="text-xs text-slate-400">Visar rad med snabbknappar ([ / ], [ /home ], etc.) vid enhetsväljaren</p>
              </div>
              <input
                type="checkbox"
                checked={$settings.showDriveButtons}
                on:change={(e) => settings.update((s) => ({ ...s, showDriveButtons: e.currentTarget.checked }))}
                class="w-4 h-4 accent-blue-500 rounded border-slate-700 bg-slate-900 cursor-pointer"
              />
            </label>
          </div>
        </div>

        <!-- File Management Section -->
        <div class="space-y-3">
          <h3 class="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider flex items-center gap-1.5">
            <Trash2 class="w-4 h-4 text-rose-400" />
            Filhantering
          </h3>

          <div class="bg-slate-950/70 border border-slate-800 rounded-lg p-3">
            <!-- Toggle confirmOnDelete -->
            <label class="flex items-center justify-between cursor-pointer group">
              <div class="space-y-0.5">
                <span class="font-medium text-slate-200 group-hover:text-rose-400 transition-colors">Bekräfta vid radering (F8)</span>
                <p class="text-xs text-slate-400">Visa bekräftelsedialogruta innan filer och mappar raderas</p>
              </div>
              <input
                type="checkbox"
                checked={$settings.confirmOnDelete}
                on:change={(e) => settings.update((s) => ({ ...s, confirmOnDelete: e.currentTarget.checked }))}
                class="w-4 h-4 accent-rose-500 rounded border-slate-700 bg-slate-900 cursor-pointer"
              />
            </label>
          </div>
        </div>

        <!-- Default Paths Section -->
        <div class="space-y-3">
          <h3 class="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider flex items-center gap-1.5">
            <Folder class="w-4 h-4 text-amber-400" />
            Standard Sökvägar Vid Start
          </h3>

          <div class="bg-slate-950/70 border border-slate-800 rounded-lg p-3 space-y-3 font-mono text-xs">
            <div>
              <label class="block text-slate-400 mb-1 font-sans font-medium">Vänster Panel Standard-sökväg:</label>
              <input
                type="text"
                value={$settings.defaultLeftPath}
                on:input={(e) => settings.update((s) => ({ ...s, defaultLeftPath: e.currentTarget.value }))}
                class="w-full bg-slate-900 border border-slate-700 rounded px-3 py-1.5 text-slate-100 outline-none focus:border-blue-500"
              />
            </div>
            <div>
              <label class="block text-slate-400 mb-1 font-sans font-medium">Höger Panel Standard-sökväg:</label>
              <input
                type="text"
                value={$settings.defaultRightPath}
                on:input={(e) => settings.update((s) => ({ ...s, defaultRightPath: e.currentTarget.value }))}
                class="w-full bg-slate-900 border border-slate-700 rounded px-3 py-1.5 text-slate-100 outline-none focus:border-blue-500"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-between pt-3 border-t border-slate-800 flex-shrink-0">
        <button
          type="button"
          on:click={handleReset}
          class="px-3 py-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-slate-200 text-xs font-mono flex items-center gap-1.5 transition-colors"
        >
          <RotateCcw class="w-3.5 h-3.5" />
          <span>Återställ standard</span>
        </button>

        <button
          type="button"
          on:click={handleClose}
          class="px-4 py-2 rounded bg-blue-600 hover:bg-blue-500 text-white font-semibold text-xs flex items-center gap-1.5 shadow-lg shadow-blue-600/30 transition-all"
        >
          <Check class="w-4 h-4" />
          <span>Klar</span>
        </button>
      </div>
    </div>
  </div>
{/if}
