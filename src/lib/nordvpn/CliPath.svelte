<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let output = false;

  async function getCliPath() {
    let res = await invoke("nordvpn_cli_path", {});
    output = res;
  }

  onMount(async () => {
    await getCliPath();
  });
</script>

<div>{output}</div>

{#if output} {/if}
