<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let output = {}

  async function getListOfCountries(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let res = await invoke("get_nordvpn_countries", {});
    // output = JSON.parse(JSON.parse(res));
    output = res;
    output = JSON.parse(res);
  }
</script>

<div>
  <button on:click={getListOfCountries}>
    Get List of Countries
  </button>
  <pre>{output}</pre>
</div>