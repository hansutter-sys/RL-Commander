<script lang="ts">
  export let onView: () => void;
  export let onEdit: () => void;
  export let onCopy: () => void;
  export let onMove: () => void;
  export let onCreateDir: () => void;
  export let onDelete: () => void;

  const buttons = [
    { key: "F3", label: "Visa", action: onView, color: "hover:bg-sky-600 border-sky-700/50" },
    { key: "F4", label: "Redigera", action: onEdit, color: "hover:bg-amber-600 border-amber-700/50" },
    { key: "F5", label: "Kopiera", action: onCopy, color: "hover:bg-emerald-600 border-emerald-700/50" },
    { key: "F6", label: "Flytta", action: onMove, color: "hover:bg-purple-600 border-purple-700/50" },
    { key: "F7", label: "Ny Mapp", action: onCreateDir, color: "hover:bg-blue-600 border-blue-700/50" },
    { key: "F8", label: "Radera", action: onDelete, color: "hover:bg-rose-600 border-rose-700/50" },
  ];
</script>

<div class="bg-slate-950 p-1.5 border-t border-slate-800 grid grid-cols-6 gap-1 select-none font-mono">
  {#each buttons as btn}
    <button
      on:click={btn.action}
      class="bg-slate-900 border border-slate-800 text-slate-200 py-1.5 px-2 rounded flex items-center justify-center gap-1.5 transition-all text-xs font-semibold shadow-sm {btn.color} hover:text-white active:scale-95"
    >
      <span class="bg-slate-800 text-blue-400 px-1.5 py-0.5 rounded text-[10px] font-bold border border-slate-700">
        {btn.key}
      </span>
      <span>{btn.label}</span>
    </button>
  {/each}
</div>
