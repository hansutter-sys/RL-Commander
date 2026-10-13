<script lang="ts">
  import { settings } from "../stores/settings";

  export let onView: () => void;
  export let onEdit: () => void;
  export let onCopy: () => void;
  export let onMove: () => void;
  export let onCreateDir: () => void;
  export let onDelete: () => void;

  const buttons = [
    { key: "F3", label: "Visa", action: onView, color: "hover:bg-sky-600 hover:text-white border-sky-700/50" },
    { key: "F4", label: "Redigera", action: onEdit, color: "hover:bg-amber-600 hover:text-white border-amber-700/50" },
    { key: "F5", label: "Kopiera", action: onCopy, color: "hover:bg-emerald-600 hover:text-white border-emerald-700/50" },
    { key: "F6", label: "Flytta", action: onMove, color: "hover:bg-purple-600 hover:text-white border-purple-700/50" },
    { key: "F7", label: "Ny Mapp", action: onCreateDir, color: "hover:bg-blue-600 hover:text-white border-blue-700/50" },
    { key: "F8", label: "Radera", action: onDelete, color: "hover:bg-rose-600 hover:text-white border-rose-700/50" },
  ];
</script>

<div
  class="p-1.5 border-t grid grid-cols-6 gap-1 select-none font-mono transition-colors
    {$settings.theme === 'light' ? 'bg-slate-100 border-slate-300' : 'bg-slate-950 border-slate-800'}"
>
  {#each buttons as btn}
    <button
      on:click={btn.action}
      class="border py-1.5 px-2 rounded flex items-center justify-center gap-1.5 transition-all text-xs font-semibold shadow-2xs active:scale-95
        {$settings.theme === 'light'
          ? 'bg-white border-slate-300 text-slate-700 hover:bg-slate-50'
          : 'bg-slate-900 border-slate-800 text-slate-200 hover:text-white'}
        {btn.color}"
    >
      <span
        class="px-1.5 py-0.5 rounded text-[10px] font-bold border
          {$settings.theme === 'light'
            ? 'bg-slate-100 text-blue-600 border-slate-200'
            : 'bg-slate-800 text-blue-400 border-slate-700'}"
      >
        {btn.key}
      </span>
      <span>{btn.label}</span>
    </button>
  {/each}
</div>
