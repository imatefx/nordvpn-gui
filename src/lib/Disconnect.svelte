<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let output = {}

  async function disconnect(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let res = await invoke("nordvpn_disconnect", {});
    // output = JSON.parse(JSON.parse(res));
    output = res;
    // output = JSON.parse(res);
  }
</script>

<div>
  <button on:click={disconnect}>
    Disconnect
  </button>
  <pre>{output}</pre>
</div>