<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri"
  import DisconnectServer from "../nordvpn/DisconnectServer.svelte";


</script>

  <div style="text-align: left">
    <DisconnectServer on:SERVER_DISCONNECTION_ATTEMPTED/>
  </div>