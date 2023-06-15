<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri"
  import ConnectServer from "../nordvpn/ConnectServer.svelte";
</script>

<div style="text-align: left">
  <ConnectServer connectType="RANDOM" connectLabel="Connect Random Server" on:SERVER_CONNECTION_ATTEMPTED />
</div>
