<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let output = {}

  async function connectRandomServer(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // let res = await invoke("get_nordvpn_account_info", {})
    // let res = await invoke("get_nordvpn_countries", {});
    let res = await invoke("nordvpn_connect_random", {});
    // output = JSON.parse(JSON.parse(res));
    output = res;
    // output = JSON.parse(res);
  }
</script>

<div>
  <button on:click={connectRandomServer}>
    Connect Random Server
  </button>
  <pre>{output}</pre>
</div>