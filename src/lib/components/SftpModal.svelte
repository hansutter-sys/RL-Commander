<script lang="ts">
  import { showSftpModal, connectSftp, activePane } from "../stores/commander";
  import { settings } from "../stores/settings";
  import { Server, X } from "lucide-svelte";

  let host = "";
  let port = 22;
  let username = "";
  let password = "";
  let privateKeyPath = "";

  function handleConnect() {
    if (!host || !username) return;
    connectSftp($activePane, {
      host,
      port,
      username,
      password: password || undefined,
      private_key_path: privateKeyPath || undefined,
    });
    showSftpModal.set(false);
  }
</script>

{#if $showSftpModal}
  <div class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div
      class="border rounded-xl w-full max-w-md shadow-2xl p-5 font-sans transition-colors
        {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-900 border-slate-700 text-slate-100'}"
    >
      <div class="flex items-center justify-between pb-3 border-b {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
        <div class="flex items-center gap-2 font-bold text-lg text-purple-500">
          <Server class="w-5 h-5" />
          <span>Anslut SFTP Server</span>
        </div>
        <button
          on:click={() => showSftpModal.set(false)}
          class="{$settings.theme === 'light' ? 'text-slate-400 hover:text-slate-700' : 'text-slate-400 hover:text-white'}"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <form on:submit|preventDefault={handleConnect} class="space-y-4 mt-4 font-mono text-xs">
        <div>
          <label for="sftp-host-input" class="block mb-1 {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
            Värdadress / Host
          </label>
          <input
            id="sftp-host-input"
            type="text"
            bind:value={host}
            placeholder="example.com eller 192.168.1.100"
            required
            class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
              {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
          />
        </div>

        <div class="grid grid-cols-3 gap-3">
          <div class="col-span-2">
            <label for="sftp-user-input" class="block mb-1 {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
              Användarnamn
            </label>
            <input
              id="sftp-user-input"
              type="text"
              bind:value={username}
              placeholder="root / admin"
              required
              class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
            />
          </div>
          <div>
            <label for="sftp-port-input" class="block mb-1 {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
              Port
            </label>
            <input
              id="sftp-port-input"
              type="number"
              bind:value={port}
              class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
                {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
            />
          </div>
        </div>

        <div>
          <label for="sftp-pass-input" class="block mb-1 {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
            Lösenord (Valfritt)
          </label>
          <input
            id="sftp-pass-input"
            type="password"
            bind:value={password}
            placeholder="••••••••"
            class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
              {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
          />
        </div>

        <div>
          <label for="sftp-key-input" class="block mb-1 {$settings.theme === 'light' ? 'text-slate-600' : 'text-slate-400'}">
            Privat nyckel (Sökväg)
          </label>
          <input
            id="sftp-key-input"
            type="text"
            bind:value={privateKeyPath}
            placeholder="~/.ssh/id_rsa"
            class="w-full border rounded px-3 py-2 outline-none focus:border-purple-500
              {$settings.theme === 'light' ? 'bg-white border-slate-300 text-slate-800' : 'bg-slate-950 border-slate-700 text-slate-100'}"
          />
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t {$settings.theme === 'light' ? 'border-slate-200' : 'border-slate-800'}">
          <button
            type="button"
            on:click={() => showSftpModal.set(false)}
            class="px-4 py-2 rounded font-semibold transition-colors
              {$settings.theme === 'light' ? 'bg-slate-100 hover:bg-slate-200 text-slate-700' : 'bg-slate-800 hover:bg-slate-700 text-slate-300'}"
          >
            Avbryt
          </button>
          <button
            type="submit"
            class="px-4 py-2 rounded bg-purple-600 hover:bg-purple-500 text-white font-semibold flex items-center gap-1.5 shadow-lg shadow-purple-600/30"
          >
            <Server class="w-4 h-4" />
            Anslut Pane
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
