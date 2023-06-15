<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import DisconnectServer from "./DisconnectServer.svelte";


  let statusOutput = false;

  async function getStatus() {
    let res = await invoke("get_nordvpn_status", {});
    statusOutput = JSON.parse(JSON.parse(res));
  }

  onMount(async () => {
    await getStatus();
  });
</script>

<div class="row">

  <table style="float: left; width: 400px; text-align: left;">
    <tr>
      <th>Status</th>
      <td>{statusOutput.status}</td>
    </tr>
    <tr>
      <th>Hostname</th>
      <td>{statusOutput.hostname || ""}</td>
    </tr>

    <tr>
      <th>IP</th>
      <td>{statusOutput.ip || ""}</td>
    </tr>
    <tr>
      <th>Country</th>
      <td>{statusOutput.country || ""}</td>
    </tr>
    <tr>
      <th>City</th>
      <td>{statusOutput.city || ""}</td>
    </tr>
  </table>
<div style="width: 30px"></div>
  <table style="float: left; width: 400px; text-align: left;">
    <tr>
      <th>Current Technology</th>
      <td>{statusOutput.currentTechnology || ""}</td>
    </tr>
    <tr>
      <th>Current Protocol</th>
      <td>{statusOutput.currentProtocol || ""}</td>
    </tr>
    <tr>
      <th>Transfer</th>
      <td>{statusOutput.transfer || ""}</td>
    </tr>
    <tr>
      <th>Uptime</th>
      <td>{statusOutput.uptime || ""}</td>
    </tr>
    <tr>
      <th><DisconnectServer on:SERVER_DISCONNECTION_ATTEMPTED /></th>
      <td><button on:click="{getStatus}" style="width:100%">Refresh</button></td>
    </tr>
  </table>
</div>
{#if statusOutput} {/if}
<style>
    table{
    width: 100%;
    border-collapse:collapse;
    table-layout: fixed;
    border: 1px solid black;
}

th{
    width: 50%;
    text-align: left;
    border: 1px solid black;
    line-height: 2;
     padding: 15px;
}

td{
    width: 50%;
    text-align: left;
    border: 1px solid black;
    line-height: 2;
     padding: 15px;
}


</style>