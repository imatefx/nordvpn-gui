<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri"
  import Status from "../nordvpn/Status.svelte";


</script>

  <!-- <div style="text-align: left"> -->
    <Status on:SERVER_DISCONNECTION_ATTEMPTED on:SERVER_CONNECTION_ATTEMPTED/>
  <!-- </div> -->