<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";

  let cities = [];
  let selectedCity = {};
  let answer = "";
  export let selectedCountry = "";

  const dispatch = createEventDispatcher();

  function onSelectedCityChangedHandler() {
    dispatch("selectedCityChanged", selectedCity);
  }

  async function getListOfCities() {
    if (selectedCountry.id) {
      let res = await invoke("get_nordvpn_cities", {
        country: selectedCountry.id,
      });
      cities = JSON.parse(JSON.parse(res));
    }
  }

  onMount(async () => {
    await getListOfCities();
  });
</script>

<div>
  <!-- <button on:click="{getListOfCities}">Get List of Cities</button> -->
  <select
    bind:value="{selectedCity}"
    on:change="{onSelectedCityChangedHandler}"
  >
    {#each cities as city}
    <option value="{city}">{city.name}</option>
    {/each}
  </select>

</div>
