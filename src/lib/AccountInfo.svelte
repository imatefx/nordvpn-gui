<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let accountInfoOutput = {}

  async function getAccountInfo(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let res = await invoke("get_nordvpn_account_info", {})
    // accountInfoOutput = JSON.parse(JSON.parse(res));
    accountInfoOutput = JSON.parse(res);
  }
</script>

<div>
  <button on:click={getAccountInfo}>
    Get Account Info
  </button>
  <pre>{accountInfoOutput}</pre>
  <div>{accountInfoOutput.hasUpdate}</div>
  <pre>{accountInfoOutput.email}</pre>
  <pre>{accountInfoOutput.vpnServiceStatus}</pre>
  <pre>{accountInfoOutput.expiresOn}</pre>
</div>