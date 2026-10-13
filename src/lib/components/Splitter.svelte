<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { GripVertical } from "lucide-svelte";

  export let isDragging = false;
  const dispatch = createEventDispatcher<{
    startResize: MouseEvent;
    resetResize: void;
  }>();

  function handleMouseDown(e: MouseEvent) {
    e.preventDefault();
    dispatch("startResize", e);
  }

  function handleDblClick() {
    dispatch("resetResize");
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  on:mousedown={handleMouseDown}
  on:dblclick={handleDblClick}
  title="Dra för att ändra panelbredd (dubbelklicka för att centrera 50/50)"
  class="w-2 hover:w-2.5 cursor-col-resize select-none flex items-center justify-center transition-all z-20 group relative h-full flex-shrink-0
    {isDragging ? 'bg-blue-500 w-2.5 shadow-lg shadow-blue-500/50' : 'bg-transparent hover:bg-blue-600/40'}"
>
  <div class="h-10 w-1.5 rounded-full flex items-center justify-center transition-colors {isDragging ? 'bg-white' : 'bg-slate-700 group-hover:bg-blue-400'}">
    <GripVertical class="w-2.5 h-2.5 {isDragging ? 'text-blue-600' : 'text-slate-400 group-hover:text-white'}" />
  </div>
</div>
