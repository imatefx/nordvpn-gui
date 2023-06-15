<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  export let logs = [];
</script>

<h2>Logs</h2>
<div style="text-align: left">
  <ul>
    {#each logs as log, i}
    <li>{i + 1}: {@html log}</li>
    {/each}
  </ul>
</div>
