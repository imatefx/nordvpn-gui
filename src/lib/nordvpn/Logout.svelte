<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let output = false;

  async function handlerFn() {
    let res = await invoke("nordvpn_logout", {});
    output = res;
  }
</script>

<div class="row">
  <button on:click="{handlerFn}">Logout</button>
</div>
<div class="row">
  {#if output}
  <p>{@html output}</p>
  {/if}
</div>
