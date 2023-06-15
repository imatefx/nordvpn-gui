<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  export let connectType = "RANDOM";
  export let connectLabel = "Random Server";
  export let city = "";
  export let country = "";
  export let group = "";

  let output = {};

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function emitServerConnectionAttempt(data) {
    dispatch("SERVER_CONNECTION_ATTEMPTED", { response: data.response });
  }

  async function connectRandomServer() {
    output = await invoke("nordvpn_connect_random", {});
    emitServerConnectionAttempt({ response: output });
  }

  async function connectCountryServer() {
    output = await invoke("nordvpn_connect_country", { country: country });
    emitServerConnectionAttempt({ response: output });
  }

  async function connectCityServer() {
    output = await invoke("nordvpn_connect_city", {
      country: country,
      city: city,
    });
    emitServerConnectionAttempt({ response: output });
  }

  async function connectGroupServer() {
    output = await invoke("nordvpn_connect_group", { group: group });
    emitServerConnectionAttempt({ response: output });
  }

  async function connectServerBasedOnType() {
    if (connectType == "RANDOM") {
      await connectRandomServer();
    } else if (connectType == "COUNTRY") {
      await connectCountryServer();
    } else if (connectType == "CITY") {
      await connectCityServer();
    } else if (connectType == "GROUP") {
      await connectGroupServer();
    }
  }
</script>

<div>
  <button on:click="{connectServerBasedOnType}">{connectLabel}</button>
  <!-- <pre>{@html output}</pre> -->
</div>
