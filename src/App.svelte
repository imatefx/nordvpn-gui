<script lang="ts">
  import Tabs from "./lib/components/Tabs.svelte";
  import AccountInfoSection from "./lib/sections/AccountInfoSection.svelte";
  import StatusSection from "./lib/sections/StatusSection.svelte";
  import ConnectRandomServerSection from "./lib/sections/ConnectRandomServerSection.svelte";
  import DisconnectServerSection from "./lib/sections/DisconnectServerSection.svelte";
  import LogViewerSection from "./lib/sections/LogViewerSection.svelte";
  import SystemInfoSection from "./lib/sections/SystemInfoSection.svelte";
  import ConnectTabSection from "./lib/sections/ConnectTabSection.svelte";

    import LoginWithBrowser from "./lib/nordvpn/LoginWithBrowser.svelte";
  import LoginWithToken from "./lib/nordvpn/LoginWithToken.svelte";
  import Logout from "./lib/nordvpn/Logout.svelte";

  let logs = [];
  let refreshUnique = {};
  function onConnectionChangeHandler(event) {
    console.log(event.detail);
    let str = new Date().toLocaleString().concat(" : ", event.detail.response);
    logs.push(str);
    refreshUnique = {};
  }


 let accountSectionItems = [
    { label: "Connection Info", value: 1, component: StatusSection },
    { label: "Connect", value: 2, component: ConnectTabSection },
    { label: "System Info", value: 3, component: SystemInfoSection },
    { label: "Account Info", value: 4, component: AccountInfoSection },
    { label: "Login with browser", value: 5, component: LoginWithBrowser },
    { label: "Login with Token", value: 6, component: LoginWithToken },
    { label: "Logout", value: 7, component: Logout },
  ];


</script>

<main class="container">
  <h1>NordVPN GUI for Linux</h1>

<div class="row">
  <p>
<ConnectRandomServerSection  on:SERVER_CONNECTION_ATTEMPTED="{onConnectionChangeHandler}"/>
  </p>
  <p>
<DisconnectServerSection  on:SERVER_DISCONNECTION_ATTEMPTED="{onConnectionChangeHandler}"/>
  </p>
    
</div>

  <br />
  {#key refreshUnique}
   <Tabs items={accountSectionItems} on:SERVER_CONNECTION_ATTEMPTED="{onConnectionChangeHandler}" />
  {/key}

  {#key refreshUnique}
  <LogViewerSection logs="{logs}" />
  {/key}
</main>

<style>

</style>
