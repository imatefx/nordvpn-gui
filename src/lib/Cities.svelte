<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";

  let cities = [];
  let selectedCity = {};
  let answer = "";
    export let selectedCountry = "";

  const dispatch = createEventDispatcher();

  function onSelectedCityChangedHandler() {
    dispatch("selectedCityChanged", selectedCity);
  }

  async function getListOfCities() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let res = await invoke("get_nordvpn_cities", { country: selectedCountry.id });
    cities = JSON.parse(JSON.parse(res));
    // cities = res;
    // cities = JSON.parse(res);
  }
</script>

<div>
  <button on:click="{getListOfCities}">Get List of Cities</button>
  <pre>cities : {cities}</pre>
  <pre>name: {selectedCountry.name}</pre>
  <select bind:value="{selectedCity}" on:change="{onSelectedCityChangedHandler}">
    {#each cities as city}
    <option value="{city}">{city.name}</option>
    {/each}
  </select>
  <ul></ul>
</div>
