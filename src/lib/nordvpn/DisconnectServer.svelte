<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let output = {};
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function emitServerDisconnectionAttempt(data) {
    dispatch("SERVER_DISCONNECTION_ATTEMPTED", { response: data.response });
  }

  async function disconnect() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    output = await invoke("nordvpn_disconnect", {});
    emitServerDisconnectionAttempt({ response: output });
    // output = JSON.parse(JSON.parse(res));
    // output = res;
    // output = JSON.parse(res);
  }
</script>

<div>
  <button style="width:100%" on:click="{disconnect}">Disconnect</button>
  <!-- <pre>{@html output}</pre> -->
</div>
