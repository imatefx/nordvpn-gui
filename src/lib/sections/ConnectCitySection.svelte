<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri"
  import Countries from "../nordvpn/Countries.svelte";
  import Cities from "../nordvpn/Cities.svelte";
  import ConnectServer from "../nordvpn/ConnectServer.svelte";

  let selectedCountry = {};
  let selectedCity = {};
  function onSelectedCountryChangedMessageHandler(event) {
    selectedCountry = event.detail;
  }
  function onSelectedCityChangedMessageHandler(event) {
    selectedCity = event.detail;
  }
</script>

<div style="text-align: left">
  Country : <Countries
    on:selectedCountryChanged="{onSelectedCountryChangedMessageHandler}"
  />
  {#key selectedCountry}
  City : <Cities
    selectedCountry="{selectedCountry}"
    on:selectedCityChanged="{onSelectedCityChangedMessageHandler}"
  />
  {/key}
  <br />
  <ConnectServer
    connectType="CITY"
    connectLabel="Connect Server in City"
    city="{selectedCity.id}"
    country="{selectedCountry.id}"
    on:SERVER_CONNECTION_ATTEMPTED
  />
  <!-- <p>Country : {selectedCountry.id}</p> -->
  <!-- <p>City : {selectedCity.id}</p> -->
</div>
