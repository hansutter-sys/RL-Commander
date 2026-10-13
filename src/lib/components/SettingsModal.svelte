<script lang="ts">
  import { showSettingsModal } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { Settings, X, HardDrive, Eye, Trash2, Folder, RotateCcw, Check, Monitor, Moon, Sun } from "lucide-svelte";

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
    <div
      class="border rounded-xl w-full max-w-lg shadow-2xl p-5 font-sans flex flex-col max-h-[90vh] transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between pb-3 border-b flex-shrink-0
          {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}"
      >
        <div class="flex items-center gap-2 font-bold text-lg text-blue-500">
          <Settings class="w-5 h-5 text-blue-500" />
          <span>Inställningar</span>
        </div>
        <button
          on:click={handleClose}
          class="p-1 rounded transition-colors
            {$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700 hover:bg-slate-100' : 'text-slate-400 hover:text-white hover:bg-slate-800'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Settings Content -->
      <div class="space-y-5 my-4 overflow-y-auto pr-1 flex-1 font-sans text-sm">
        <!-- Theme & Appearance Section -->
        <div class="space-y-3">
          <h3
            class="text-xs font-mono font-bold uppercase tracking-wider flex items-center gap-1.5
              {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}"
          >
            {#if $settings.theme === 'light'}
              <Sun class="w-4 h-4 text-amber-500" />
            {:else}
              <Moon class="w-4 h-4 text-indigo-400" />
            {/if}
            Tema & Utseende
          </h3>

          <div
            class="border rounded-lg p-3 grid grid-cols-2 gap-3
              {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950/70 border-slate-800'}"
          >
            <!-- Dark Mode Option -->
            <button
              type="button"
              on:click={() => settings.update((s) => ({ ...s, theme: "dark" }))}
              class="flex items-center gap-3 p-3 rounded-lg border transition-all text-left
                {$settings.theme === 'dark'
                  ? 'bg-blue-600/20 border-blue-500 ring-2 ring-blue-500/50 text-white'
                  : 'bg-slate-900 border-slate-800 text-slate-400 hover:border-slate-700'}"
            >
              <div class="p-2 rounded-md bg-slate-800 text-indigo-300">
                <Moon class="w-5 h-5" />
              </div>
              <div>
                <div class="font-semibold text-xs text-slate-100">Mörkt läge</div>
                <div class="text-[11px] text-slate-400">Klassisk mörk Commander-stil</div>
              </div>
            </button>

            <!-- Light Mode Option -->
            <button
              type="button"
              on:click={() => settings.update((s) => ({ ...s, theme: "light" }))}
              class="flex items-center gap-3 p-3 rounded-lg border transition-all text-left
                {$settings.theme === 'light'
                  ? 'bg-blue-50 border-blue-500 ring-2 ring-blue-500/50 text-slate-900'
                  : 'bg-slate-800/60 border-slate-700 text-slate-400 hover:border-slate-600'}"
            >
              <div class="p-2 rounded-md bg-amber-100 text-amber-600">
                <Sun class="w-5 h-5" />
              </div>
              <div>
                <div class="font-semibold text-xs {$settings.theme === 'light' ? 'text-slate-900' : 'text-slate-200'}">Ljust läge</div>
                <div class="text-[11px] {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400'}">Ljus bakgrund & hög kontrast</div>
              </div>
            </button>
          </div>
        </div>

        <!-- Layout & Display Section -->
        <div class="space-y-3">
          <h3
            class="text-xs font-mono font-bold uppercase tracking-wider flex items-center gap-1.5
              {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}"
          >
            <Monitor class="w-4 h-4 text-indigo-400" />
            Visning & Panelvy
          </h3>

          <div
            class="border rounded-lg p-3 space-y-3
              {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950/70 border-slate-800'}"
          >
            <!-- Toggle showDiskSpace -->
            <label class="flex items-center justify-between cursor-pointer group">
              <div class="space-y-0.5">
                <span class="font-medium group-hover:text-blue-500 transition-colors {$settings.theme === 'light' ? 'text-slate-800' : 'text-slate-200'}">
                  Visa diskutrymme
                </span>
                <p class="text-xs {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400'}">
                  Visar disk-användning, ledigt utrymme och procentbar under enhetsväljaren
                </p>
              </div>
              <input
                type="checkbox"
                checked={$settings.showDiskSpace}
                on:change={(e) => settings.update((s) => ({ ...s, showDiskSpace: e.currentTarget.checked }))}
                class="w-4 h-4 accent-blue-500 rounded cursor-pointer"
              />
            </label>

            <div class="h-px {$settings.theme === 'light' ? 'bg-slate-200' : 'bg-slate-800/60'}"></div>

            <!-- Toggle showDriveButtons -->
            <label class="flex items-center justify-between cursor-pointer group">
              <div class="space-y-0.5">
                <span class="font-medium group-hover:text-blue-500 transition-colors {$settings.theme === 'light' ? 'text-slate-800' : 'text-slate-200'}">
                  Visa snabbknappar för diskar
                </span>
                <p class="text-xs {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400'}">
                  Visar rad med snabbknappar ([ / ], [ /home ], etc.) vid enhetsväljaren
                </p>
              </div>
              <input
                type="checkbox"
                checked={$settings.showDriveButtons}
                on:change={(e) => settings.update((s) => ({ ...s, showDriveButtons: e.currentTarget.checked }))}
                class="w-4 h-4 accent-blue-500 rounded cursor-pointer"
              />
            </label>
          </div>
        </div>

        <!-- File Management Section -->
        <div class="space-y-3">
          <h3
            class="text-xs font-mono font-bold uppercase tracking-wider flex items-center gap-1.5
              {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}"
          >
            <Trash2 class="w-4 h-4 text-rose-400" />
            Filhantering
          </h3>

          <div
            class="border rounded-lg p-3
              {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950/70 border-slate-800'}"
          >
            <!-- Toggle confirmOnDelete -->
            <label class="flex items-center justify-between cursor-pointer group">
              <div class="space-y-0.5">
                <span class="font-medium group-hover:text-rose-500 transition-colors {$settings.theme === 'light' ? 'text-slate-800' : 'text-slate-200'}">
                  Bekräfta vid radering (F8)
                </span>
                <p class="text-xs {$settings.theme === 'light' ? 'text-slate-500' : 'text-slate-400'}">
                  Visa bekräftelsedialogruta innan filer och mappar raderas
                </p>
              </div>
              <input
                type="checkbox"
                checked={$settings.confirmOnDelete}
                on:change={(e) => settings.update((s) => ({ ...s, confirmOnDelete: e.currentTarget.checked }))}
                class="w-4 h-4 accent-rose-500 rounded cursor-pointer"
              />
            </label>
          </div>
        </div>

        <!-- Default Paths Section -->
        <div class="space-y-3">
          <h3
            class="text-xs font-mono font-bold uppercase tracking-wider flex items-center gap-1.5
              {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}"
          >
            <Folder class="w-4 h-4 text-amber-500" />
            Standard Sökvägar Vid Start
          </h3>

          <div
            class="border rounded-lg p-3 space-y-3 font-mono text-xs
              {$settings.theme === 'light' ? 'bg-slate-50 border-slate-200' : 'bg-slate-950/70 border-slate-800'}"
          >
            <div>
              <label for="default-left-path" class="block mb-1 font-sans font-medium {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
                Vänster Panel Standard-sökväg:
              </label>
              <input
                id="default-left-path"
                type="text"
                value={$settings.defaultLeftPath}
                on:input={(e) => settings.update((s) => ({ ...s, defaultLeftPath: e.currentTarget.value }))}
                class="w-full border rounded px-3 py-1.5 outline-none focus:border-blue-500
                  {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
              />
            </div>
            <div>
              <label for="default-right-path" class="block mb-1 font-sans font-medium {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
                Höger Panel Standard-sökväg:
              </label>
              <input
                id="default-right-path"
                type="text"
                value={$settings.defaultRightPath}
                on:input={(e) => settings.update((s) => ({ ...s, defaultRightPath: e.currentTarget.value }))}
                class="w-full border rounded px-3 py-1.5 outline-none focus:border-blue-500
                  {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-between pt-3 border-t flex-shrink-0
          {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}"
      >
        <button
          type="button"
          on:click={handleReset}
          class="px-3 py-1.5 rounded text-xs font-mono flex items-center gap-1.5 transition-colors
            {$settings.theme === 'light'
              ? 'bg-slate-100 hover:bg-slate-200 text-slate-600'
              : 'bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-slate-200'}"
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
