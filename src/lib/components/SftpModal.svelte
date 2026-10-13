<script lang="ts">
  import { onMount } from "svelte";
  import { showSftpModal, connectSftp, activePane, showToast } from "../stores/commander";
  import { settings } from "../stores/settings";
  import {
    sftpProfiles,
    isSftpUnlocked,
    initSftpProfiles,
    unlockWithPin,
    lockSftpProfiles,
    addOrUpdateSftpProfile,
    deleteSftpProfile,
    profileToSftpConfig,
  } from "../stores/sftpProfiles";
  import type { SftpProfile } from "../types";
  import {
    Server,
    X,
    Lock,
    Unlock,
    Plus,
    Trash2,
    Edit3,
    ArrowRight,
    Key,
    ShieldCheck,
    Eye,
    EyeOff,
    Check,
    ChevronLeft,
  } from "lucide-svelte";

  // Mode: "list" | "form" | "pin"
  let mode: "list" | "form" = "list";
  let showManualOnly = false;

  // PIN Unlock state
  let enteredPin = "";
  let pinError = "";

  // Form state
  let editingProfileId: string | null = null;
  let formName = "";
  let formHost = "";
  let formPort = 22;
  let formUsername = "";
  let formPassword = "";
  let formPrivateKeyPath = "";
  let showPasswordPlain = false;

  onMount(async () => {
    await initSftpProfiles();
  });

  $: requiresPinUnlock = $settings.pinProtectionEnabled && !$isSftpUnlocked && !showManualOnly;

  async function handleUnlockPin() {
    if (!enteredPin) return;
    pinError = "";
    const success = await unlockWithPin(enteredPin);
    if (success) {
      enteredPin = "";
      mode = "list";
      showToast("SFTP-inloggningar upplåsta med PIN");
    } else {
      pinError = "Felaktig PIN-kod. Försök igen.";
    }
  }

  function handleStartNewProfile() {
    editingProfileId = null;
    formName = "";
    formHost = "";
    formPort = 22;
    formUsername = "";
    formPassword = "";
    formPrivateKeyPath = "";
    showPasswordPlain = false;
    mode = "form";
  }

  function handleEditProfile(p: SftpProfile) {
    editingProfileId = p.id;
    formName = p.name;
    formHost = p.host;
    formPort = p.port;
    formUsername = p.username;
    formPassword = p.password || "";
    formPrivateKeyPath = p.private_key_path || "";
    showPasswordPlain = false;
    mode = "form";
  }

  async function handleSaveProfile(andConnect = false) {
    if (!formHost.trim() || !formUsername.trim()) return;

    const profile: SftpProfile = {
      id: editingProfileId || `sftp_${Date.now()}_${Math.random().toString(36).substring(2, 7)}`,
      name: formName.trim() || `${formUsername}@${formHost}`,
      host: formHost.trim(),
      port: Number(formPort) || 22,
      username: formUsername.trim(),
      password: formPassword || undefined,
      private_key_path: formPrivateKeyPath.trim() || undefined,
      created_at: Date.now(),
    };

    await addOrUpdateSftpProfile(profile);

    if (andConnect) {
      handleConnectProfile(profile);
    } else {
      mode = "list";
    }
  }

  function handleConnectProfile(p: SftpProfile) {
    const config = profileToSftpConfig(p);
    connectSftp($activePane, config);
    showSftpModal.set(false);
    showToast(`Ansluten till ${p.name} (${p.host})`);
  }

  function handleDirectConnect() {
    if (!formHost.trim() || !formUsername.trim()) return;
    connectSftp($activePane, {
      host: formHost.trim(),
      port: Number(formPort) || 22,
      username: formUsername.trim(),
      password: formPassword || undefined,
      private_key_path: formPrivateKeyPath.trim() || undefined,
    });
    showSftpModal.set(false);
  }

  async function handleDeleteProfile(p: SftpProfile) {
    if (confirm(`Är du säker på att du vill ta bort profilen "${p.name}"?`)) {
      await deleteSftpProfile(p.id);
    }
  }

  function handleClose() {
    showSftpModal.set(false);
    showManualOnly = false;
    enteredPin = "";
    pinError = "";
  }
</script>

{#if $showSftpModal}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div
      class="border rounded-xl w-full max-w-xl shadow-2xl p-5 font-sans flex flex-col max-h-[85vh] transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <!-- Modal Header -->
      <div
        class="flex items-center justify-between pb-3 border-b flex-shrink-0 transition-colors
          {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}"
      >
        <div class="flex items-center gap-2 font-bold text-lg text-purple-500">
          <Server class="w-5 h-5 text-purple-500" />
          <span>SFTP Site Manager</span>
          {#if $settings.pinProtectionEnabled}
            <span
              class="text-[10px] font-mono px-2 py-0.5 rounded flex items-center gap-1 font-normal
                {$isSftpUnlocked
                  ? 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/30'
                  : 'bg-amber-500/10 text-amber-500 border border-amber-500/30'}"
            >
              {#if $isSftpUnlocked}
                <Unlock class="w-3 h-3" /> Upplåst
              {:else}
                <Lock class="w-3 h-3" /> PIN-låst
              {/if}
            </span>
          {/if}
        </div>
        <button
          on:click={handleClose}
          class="transition-colors {$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content Views -->
      <div class="my-4 overflow-y-auto pr-1 flex-1 text-xs font-mono">
        <!-- 1. PIN Unlock View -->
        {#if requiresPinUnlock}
          <div class="py-6 px-4 flex flex-col items-center text-center space-y-4">
            <div class="w-12 h-12 rounded-full bg-purple-500/10 border border-purple-500/30 flex items-center justify-center text-purple-500 shadow-md">
              <Lock class="w-6 h-6" />
            </div>
            <div>
              <h3 class="text-sm font-bold font-sans {$settings.theme === 'light' ? 'text-slate-900' : 'text-slate-100'}">
                PIN-kod krävs
              </h3>
              <p class="text-xs font-sans text-slate-500 mt-1 max-w-sm">
                Dina sparade SFTP-servrar och lösenord är krypterade. Ange din PIN-kod för att låsa upp profilerna.
              </p>
            </div>

            <form on:submit|preventDefault={handleUnlockPin} class="w-full max-w-xs space-y-3">
              <div>
                <input
                  type="password"
                  bind:value={enteredPin}
                  placeholder="Ange PIN-kod..."
                  maxlength="12"
                  class="w-full text-center text-lg tracking-widest px-4 py-2 rounded-lg border outline-none focus:border-purple-500
                    {$settings.theme === 'light' ? 'bg-slate-50 border-slate-300 text-slate-900' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                />
                {#if pinError}
                  <div class="text-rose-500 text-[11px] mt-1 font-sans font-semibold">
                    {pinError}
                  </div>
                {/if}
              </div>

              <div class="flex gap-2">
                <button
                  type="button"
                  on:click={() => (showManualOnly = true)}
                  class="flex-1 py-2 rounded-lg font-sans font-semibold border transition-colors
                    {$settings.theme === 'light' ? 'bg-slate-100 border-slate-200 text-slate-600 hover:bg-slate-200' : 'bg-slate-800 border-slate-700 text-slate-300 hover:bg-slate-700'}"
                >
                  Manuell anslutning
                </button>
                <button
                  type="submit"
                  class="flex-1 py-2 rounded-lg bg-purple-600 hover:bg-purple-500 text-white font-sans font-semibold shadow-lg shadow-purple-600/30 transition-all flex items-center justify-center gap-1.5"
                >
                  <Unlock class="w-4 h-4" />
                  <span>Lås upp</span>
                </button>
              </div>
            </form>
          </div>

        <!-- 2. Saved Sites List View -->
        {:else if mode === "list" && !showManualOnly}
          <div class="space-y-3">
            <div class="flex items-center justify-between pb-1">
              <span class="text-slate-500 uppercase font-bold text-[10px] tracking-wider">
                Mina Sparade Servrar ({$sftpProfiles.length})
              </span>
              <div class="flex items-center gap-1.5">
                {#if $settings.pinProtectionEnabled && $isSftpUnlocked}
                  <button
                    on:click={lockSftpProfiles}
                    title="Lås profilvalvet igen"
                    class="px-2 py-1 rounded border text-[11px] flex items-center gap-1 transition-colors
                      {$settings.theme === 'light' ? 'bg-slate-100 border-slate-300 text-slate-600 hover:bg-slate-200' : 'bg-slate-800 border-slate-700 text-slate-300 hover:bg-slate-700'}"
                  >
                    <Lock class="w-3 h-3 text-amber-500" />
                    <span>Lås nu</span>
                  </button>
                {/if}
                <button
                  on:click={handleStartNewProfile}
                  class="px-2.5 py-1 rounded bg-purple-600 hover:bg-purple-500 text-white font-semibold text-xs flex items-center gap-1 transition-all shadow-sm"
                >
                  <Plus class="w-3.5 h-3.5" />
                  <span>Lägg till server</span>
                </button>
              </div>
            </div>

            {#if $sftpProfiles.length === 0}
              <div class="text-center py-10 border-2 border-dashed rounded-xl {$settings.theme === 'light' ? 'border-slate-200 text-slate-400' : 'border-slate-800 text-slate-500'}">
                <Server class="w-8 h-8 mx-auto mb-2 opacity-40" />
                <p class="font-sans font-medium text-xs">Inga sparade SFTP-servrar än.</p>
                <p class="font-sans text-[11px] text-slate-400 mt-0.5">Klicka på "Lägg till server" för att spara en anslutning.</p>
                <button
                  on:click={handleStartNewProfile}
                  class="mt-3 px-3 py-1.5 rounded bg-purple-600 hover:bg-purple-500 text-white text-xs font-semibold inline-flex items-center gap-1"
                >
                  <Plus class="w-3.5 h-3.5" />
                  <span>Skapa profil</span>
                </button>
              </div>
            {:else}
              <div class="space-y-2">
                {#each $sftpProfiles as prof (prof.id)}
                  <div
                    class="p-3 rounded-lg border flex items-center justify-between gap-3 group transition-all
                      {$settings.theme === 'light'
                        ? 'bg-slate-50 hover:bg-purple-50/50 border-slate-200 hover:border-purple-300'
                        : 'bg-slate-950/70 hover:bg-purple-950/20 border-slate-800 hover:border-purple-800'}"
                  >
                    <div class="flex items-center gap-3 min-w-0 flex-1">
                      <div class="w-8 h-8 rounded-lg bg-purple-500/10 border border-purple-500/20 flex items-center justify-center text-purple-500 flex-shrink-0">
                        <Server class="w-4 h-4" />
                      </div>
                      <div class="truncate flex-1">
                        <div class="font-bold text-xs truncate flex items-center gap-1.5 {$settings.theme === 'light' ? 'text-slate-900' : 'text-slate-100'}">
                          <span class="truncate">{prof.name}</span>
                          {#if prof.password}
                            <span class="text-[9px] px-1.5 py-0.2 rounded bg-emerald-500/10 text-emerald-500 border border-emerald-500/20 font-mono">
                              Krypterat lösenord
                            </span>
                          {/if}
                          {#if prof.private_key_path}
                            <span class="text-[9px] px-1.5 py-0.2 rounded bg-indigo-500/10 text-indigo-500 border border-indigo-500/20 font-mono">
                              SSH-nyckel
                            </span>
                          {/if}
                        </div>
                        <div class="text-[11px] text-slate-500 font-mono truncate">
                          {prof.username}@{prof.host}:{prof.port}
                        </div>
                      </div>
                    </div>

                    <div class="flex items-center gap-1.5 flex-shrink-0">
                      <button
                        on:click={() => handleEditProfile(prof)}
                        title="Redigera profil"
                        class="p-1.5 rounded transition-colors {$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700 hover:bg-slate-200' : 'text-slate-400 hover:text-white hover:bg-slate-800'}"
                      >
                        <Edit3 class="w-3.5 h-3.5" />
                      </button>
                      <button
                        on:click={() => handleDeleteProfile(prof)}
                        title="Ta bort profil"
                        class="p-1.5 rounded text-slate-400 hover:text-rose-500 transition-colors hover:bg-rose-500/10"
                      >
                        <Trash2 class="w-3.5 h-3.5" />
                      </button>
                      <button
                        on:click={() => handleConnectProfile(prof)}
                        class="px-3 py-1.5 rounded bg-purple-600 hover:bg-purple-500 text-white font-semibold text-xs flex items-center gap-1 shadow-sm transition-all"
                      >
                        <span>Anslut</span>
                        <ArrowRight class="w-3.5 h-3.5" />
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>

        <!-- 3. Form View (Add / Edit / Manual Connect) -->
        {:else}
          <div class="space-y-4">
            <div class="flex items-center justify-between pb-1">
              <button
                on:click={() => {
                  mode = "list";
                  showManualOnly = false;
                }}
                class="text-slate-500 hover:text-purple-500 flex items-center gap-1 font-semibold text-xs"
              >
                <ChevronLeft class="w-4 h-4" />
                <span>Tillbaka till sparade servrar</span>
              </button>
              <span class="text-slate-400 font-sans text-xs">
                {editingProfileId ? "Redigera profil" : "Ny anslutning"}
              </span>
            </div>

            <div class="space-y-3">
              <div>
                <label for="profile-name" class="block mb-1 font-sans text-slate-500">
                  Profilnamn (valfritt för att känna igen servern):
                </label>
                <input
                  id="profile-name"
                  type="text"
                  bind:value={formName}
                  placeholder="t.ex. Produktionsserver, Hemma-NAS"
                  class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                    {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                />
              </div>

              <div>
                <label for="form-host" class="block mb-1 font-sans text-slate-500">
                  Värdadress / Host <span class="text-rose-500">*</span>
                </label>
                <input
                  id="form-host"
                  type="text"
                  bind:value={formHost}
                  placeholder="example.com eller 192.168.1.50"
                  required
                  class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                    {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                />
              </div>

              <div class="grid grid-cols-3 gap-3">
                <div class="col-span-2">
                  <label for="form-user" class="block mb-1 font-sans text-slate-500">
                    Användarnamn <span class="text-rose-500">*</span>
                  </label>
                  <input
                    id="form-user"
                    type="text"
                    bind:value={formUsername}
                    placeholder="root / ubuntu / admin"
                    required
                    class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                      {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                  />
                </div>
                <div>
                  <label for="form-port" class="block mb-1 font-sans text-slate-500">Port</label>
                  <input
                    id="form-port"
                    type="number"
                    bind:value={formPort}
                    class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                      {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                  />
                </div>
              </div>

              <div>
                <label for="form-pass" class="block mb-1 font-sans text-slate-500">
                  Lösenord (krypteras säkert med AES-GCM):
                </label>
                <div class="relative">
                  <input
                    id="form-pass"
                    type={showPasswordPlain ? "text" : "password"}
                    bind:value={formPassword}
                    placeholder="••••••••"
                    class="w-full border rounded px-3 py-2 pr-9 outline-none focus:border-purple-500
                      {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                  />
                  <button
                    type="button"
                    on:click={() => (showPasswordPlain = !showPasswordPlain)}
                    class="absolute right-2.5 top-2.5 text-slate-400 hover:text-slate-600"
                    title={showPasswordPlain ? "Dölj lösenord" : "Visa lösenord"}
                  >
                    {#if showPasswordPlain}
                      <EyeOff class="w-4 h-4" />
                    {:else}
                      <Eye class="w-4 h-4" />
                    {/if}
                  </button>
                </div>
              </div>

              <div>
                <label for="form-key" class="block mb-1 font-sans text-slate-500">
                  Privat SSH-nyckel (sökväg, valfritt):
                </label>
                <input
                  id="form-key"
                  type="text"
                  bind:value={formPrivateKeyPath}
                  placeholder="~/.ssh/id_rsa eller /home/user/.ssh/id_ed25519"
                  class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                    {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
                />
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer Buttons -->
      {#if !requiresPinUnlock}
        <div
          class="flex items-center justify-between pt-3 border-t flex-shrink-0 transition-colors
            {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}"
        >
          {#if mode === "list" && !showManualOnly}
            <button
              type="button"
              on:click={() => (showManualOnly = true)}
              class="px-3 py-1.5 rounded font-sans text-xs transition-colors
                {$settings.theme === 'light' ? 'bg-slate-100 hover:bg-slate-200 text-slate-600' : 'bg-slate-800 hover:bg-slate-700 text-slate-300'}"
            >
              Snabbanslut utan profil
            </button>
            <button
              type="button"
              on:click={handleClose}
              class="px-4 py-2 rounded bg-purple-600 hover:bg-purple-500 text-white font-sans font-semibold text-xs"
            >
              Stäng
            </button>
          {:else}
            <button
              type="button"
              on:click={() => {
                mode = "list";
                showManualOnly = false;
              }}
              class="px-3 py-1.5 rounded font-sans text-xs transition-colors
                {$settings.theme === 'light' ? 'bg-slate-100 hover:bg-slate-200 text-slate-600' : 'bg-slate-800 hover:bg-slate-700 text-slate-300'}"
            >
              Avbryt
            </button>
            <div class="flex items-center gap-2">
              <button
                type="button"
                on:click={handleDirectConnect}
                class="px-3 py-2 rounded border font-sans font-semibold text-xs transition-colors
                  {$settings.theme === 'light' ? 'bg-white border-purple-300 text-purple-700 hover:bg-purple-50' : 'bg-slate-800 border-purple-700 text-purple-300 hover:bg-slate-700'}"
              >
                Anslut direkt
              </button>
              <button
                type="button"
                on:click={() => handleSaveProfile(true)}
                class="px-4 py-2 rounded bg-purple-600 hover:bg-purple-500 text-white font-sans font-semibold text-xs flex items-center gap-1.5 shadow-lg shadow-purple-600/30"
              >
                <Check class="w-4 h-4" />
                <span>Spara och anslut</span>
              </button>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}
