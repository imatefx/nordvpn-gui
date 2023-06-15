<script lang="ts">
  import Tabs from "./lib/components/Tabs.svelte";
  import Greet from "./lib/Greet.svelte";
  import AccountInfoSection from "./lib/sections/AccountInfoSection.svelte";
  import StatusSection from "./lib/sections/StatusSection.svelte";
  import ConnectRandomServerSection from "./lib/sections/ConnectRandomServerSection.svelte";
  import ConnectCountrySection from "./lib/sections/ConnectCountrySection.svelte";
  import ConnectCitySection from "./lib/sections/ConnectCitySection.svelte";
  import ConnectGroupSection from "./lib/sections/ConnectGroupSection.svelte";
  import DisconnectServerSection from "./lib/sections/DisconnectServerSection.svelte";
  import LogViewerSection from "./lib/sections/LogViewerSection.svelte";

  let logs = [];
  let refreshUnique = {};
  function onConnectionChangeHandler(event) {
    console.log(event.detail);
    let str = new Date().toLocaleString().concat(" : ", event.detail.response);
    logs.push(str);
    refreshUnique = {};
  }

  // List of tab items with labels, values and assigned components
  let items = [
    { label: "Random", value: 1, component: ConnectRandomServerSection },
    { label: "Country", value: 2, component: ConnectCountrySection },
    { label: "City", value: 3, component: ConnectCitySection },
    { label: "Group", value: 4, component: ConnectGroupSection },
  ];
</script>

<main class="container">
  <h1>NordVPN GUI for Linux</h1>

  <AccountInfoSection />
  <br />
  {#key refreshUnique}
  <StatusSection
    on:SERVER_DISCONNECTION_ATTEMPTED="{onConnectionChangeHandler}"
  />
  {/key}
  <!-- <ConnectRandomServerSection />
  <ConnectCountrySection />
  <ConnectCitySection />
  <ConnectGroupSection /> -->
  <Tabs {items} on:SERVER_CONNECTION_ATTEMPTED="{onConnectionChangeHandler}" />
  <!-- <DisconnectServerSection
    on:SERVER_DISCONNECTION_ATTEMPTED="{onConnectionChangeHandler}"
  /> -->
  {#key refreshUnique}
  <LogViewerSection logs="{logs}" />
  {/key}
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
