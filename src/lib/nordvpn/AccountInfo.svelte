<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from 'svelte';

  let accountInfoOutput = false;

  async function getAccountInfo() {
    let res = await invoke("get_nordvpn_account_info", {});
    accountInfoOutput = JSON.parse(JSON.parse(res));
  }

  onMount(async () => {
    await getAccountInfo();
  });
</script>

<div class="row">
  <table style="width: 100%">
    <th>Has Update?</th>
    <th>Email ID</th>
    <th>Service Status</th>
    <th>Subscription Status</th>
    <tr>
      <td>{accountInfoOutput.hasUpdate}</td>
      <td>{accountInfoOutput.email}</td>
      <td>{accountInfoOutput.vpnServiceStatus}</td>
      <td>{accountInfoOutput.expiresOn}</td>
    </tr>
  </table>
</div>
{#if accountInfoOutput} {/if}
