<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";
      import { onMount } from 'svelte';

  let groups = [];
  let selectedGroup = {};

  const dispatch = createEventDispatcher();

  function onSelectedGroupChangedHandler() {
    dispatch("selectedGroupChanged", selectedGroup);
  }

  async function getListOfGroups() {
    let res = await invoke("get_nordvpn_groups", {});
    groups = JSON.parse(JSON.parse(res));
  }

    onMount(async () => {
    await getListOfGroups();
  });
</script>

<div>
  <!-- <button on:click="{getListOfGroups}">Get List of Groups</button> -->
  <!-- <pre>{groups}</pre> -->
  <select
    bind:value="{selectedGroup}"
    on:change="{onSelectedGroupChangedHandler}"
  >
    {#each groups as group}
    <option value="{group}">{group.name}</option>
    {/each}
  </select>
  <ul></ul>
</div>
