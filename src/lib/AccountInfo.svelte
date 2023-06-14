<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let accountInfoOutput = "Empty"

  async function getAccountInfo(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    accountInfoOutput = await invoke("get_nordvpn_account_info", {})
  }
</script>

<div>
  <button on:click={getAccountInfo}>
    Get Account Info
  </button>
  <pre>{accountInfoOutput}</pre>
</div>