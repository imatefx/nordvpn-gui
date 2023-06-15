<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri"
  import Groups from "../nordvpn/Groups.svelte";
  import ConnectServer from "../nordvpn/ConnectServer.svelte";

  let selectedGroup = {};
  function onSelectedGroupChangedMessageHandler(event) {
    selectedGroup = event.detail;
  }
</script>

<div style="text-align: left">
  <Groups on:selectedGroupChanged="{onSelectedGroupChangedMessageHandler}" />
  <ConnectServer
    connectType="GROUP"
    connectLabel="Connect Server Group"
    group="{selectedGroup.id}"
    on:SERVER_CONNECTION_ATTEMPTED
  />
</div>
