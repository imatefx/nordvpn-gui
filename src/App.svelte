<script lang="ts">
  import Greet from "./lib/Greet.svelte";
  import AccountInfo from "./lib/AccountInfo.svelte";
  import Countries from "./lib/Countries.svelte";
  import Cities from "./lib/Cities.svelte";
  import ConnectRandom from "./lib/ConnectRandom.svelte";
  import Disconnect from "./lib/Disconnect.svelte";

  let selectedCountry = {};
  function onSelectedCountryChangedMessageHandler(event) {
    selectedCountry = event.detail;
    alert(event.detail.id);
  }
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <Greet />
  </div>
  <div style="text-align: left">
    <AccountInfo />
  </div>

  <div style="text-align: left">
    <Countries on:selectedCountryChanged={onSelectedCountryChangedMessageHandler}/>
  </div>

  <div style="text-align: left">
    <Cities selectedCountry="{selectedCountry}" />
  </div>

  <div style="text-align: left">
    <ConnectRandom />
  </div>

  <div style="text-align: left">
    <Disconnect />
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
