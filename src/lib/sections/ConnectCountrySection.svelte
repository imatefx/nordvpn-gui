<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri"
  import Countries from "../nordvpn/Countries.svelte";
  import ConnectServer from "../nordvpn/ConnectServer.svelte";

  let selectedCountry = {};
  function onSelectedCountryChangedMessageHandler(event) {
    selectedCountry = event.detail;
  }
</script>

<div style="text-align: left">
  <Countries
    on:selectedCountryChanged="{onSelectedCountryChangedMessageHandler}"
  />
  <ConnectServer
    connectType="COUNTRY"
    connectLabel="Connect Server in Country"
    country="{selectedCountry.id}"
    on:SERVER_CONNECTION_ATTEMPTED
  />
</div>
