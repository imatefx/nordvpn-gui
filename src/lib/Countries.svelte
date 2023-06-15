<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";

  let countries = [];
  let selectedCountry = {};
  let answer = "";

  const dispatch = createEventDispatcher();

  function onSelectedCountryChangedHandler() {
    dispatch("selectedCountryChanged", selectedCountry);
  }

  async function getListOfCountries() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let res = await invoke("get_nordvpn_countries", {});
    countries = JSON.parse(JSON.parse(res));
    // countries = res;
    // countries = JSON.parse(res);
  }
</script>

<div>
  <button on:click="{getListOfCountries}">Get List of Countries</button>
  <pre>{countries}</pre>
  <select bind:value="{selectedCountry}" on:change="{onSelectedCountryChangedHandler}">
    {#each countries as country}
    <option value="{country}">{country.name}</option>
    {/each}
  </select>
  <ul></ul>
</div>
