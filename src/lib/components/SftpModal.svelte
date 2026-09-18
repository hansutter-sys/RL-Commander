<script lang="ts">
  import { showSftpModal, connectSftp, activePane } from "../stores/commander";
  import type { SftpConfig } from "../types";
  import { Server, Key, Lock, X } from "lucide-svelte";

  let host = "";
  let port = 22;
  let username = "";
  let password = "";
  let privateKeyPath = "";

  function handleConnect() {
    if (!host || !username) return;
    const config: SftpConfig = {
      host,
      port: Number(port),
      username,
      password: password || undefined,
      private_key_path: privateKeyPath || undefined,
    };
    connectSftp($activePane, config);
    showSftpModal.set(false);
  }
</script>

{#if $showSftpModal}
  <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700 rounded-xl w-full max-w-md shadow-2xl p-5 text-slate-100 font-sans">
      <div class="flex items-center justify-between pb-3 border-b border-slate-800">
        <div class="flex items-center gap-2 font-bold text-lg text-purple-400">
          <Server class="w-5 h-5" />
          <span>Anslut SFTP Server</span>
        </div>
        <button on:click={() => showSftpModal.set(false)} class="text-slate-400 hover:text-white">
          <X class="w-5 h-5" />
        </button>
      </div>

      <form on:submit|preventDefault={handleConnect} class="space-y-4 mt-4 font-mono text-xs">
        <div>
          <label class="block text-slate-400 mb-1">Värdadress / Host</label>
          <input
            type="text"
            bind:value={host}
            placeholder="example.com eller 192.168.1.100"
            required
            class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-purple-500"
          />
        </div>

        <div class="grid grid-cols-3 gap-3">
          <div class="col-span-2">
            <label class="block text-slate-400 mb-1">Användarnamn</label>
            <input
              type="text"
              bind:value={username}
              placeholder="root / admin"
              required
              class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-purple-500"
            />
          </div>
          <div>
            <label class="block text-slate-400 mb-1">Port</label>
            <input
              type="number"
              bind:value={port}
              class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-purple-500"
            />
          </div>
        </div>

        <div>
          <label class="block text-slate-400 mb-1">Lösenord (Valfritt)</label>
          <input
            type="password"
            bind:value={password}
            placeholder="••••••••"
            class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-purple-500"
          />
        </div>

        <div>
          <label class="block text-slate-400 mb-1">Privat nyckel (Sökväg)</label>
          <input
            type="text"
            bind:value={privateKeyPath}
            placeholder="~/.ssh/id_rsa"
            class="w-full bg-slate-950 border border-slate-700 rounded px-3 py-2 text-slate-100 outline-none focus:border-purple-500"
          />
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-slate-800">
          <button
            type="button"
            on:click={() => showSftpModal.set(false)}
            class="px-4 py-2 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 font-semibold"
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
