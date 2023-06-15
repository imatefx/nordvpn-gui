<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";
    import { onMount } from 'svelte';
  

  let countries = [];
  let selectedCountry = {};
  let answer = "";

  const dispatch = createEventDispatcher();

  function onSelectedCountryChangedHandler() {
    dispatch("selectedCountryChanged", selectedCountry);
  }

  async function getListOfCountries() {
    let res = await invoke("get_nordvpn_countries", {});
    countries = JSON.parse(JSON.parse(res));
  }

    onMount(async () => {
    await getListOfCountries();
  });

</script>

<div>
  <!-- <button on:click="{getListOfCountries}">Get List of Countries</button> -->
  <!-- <pre>{countries}</pre> -->
  <select
    bind:value="{selectedCountry}"
    on:change="{onSelectedCountryChangedHandler}"
  >
    {#each countries as country}
    <option value="{country}">{country.name}</option>
    {/each}
  </select>
  <ul></ul>
</div>
