<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let output = false;
  let token = "";

  async function handlerFn() {
    let res = await invoke("nordvpn_login_using_token", { token: token });
    output = res;
  }
</script>

<form class="row" on:submit|preventDefault="{handlerFn}">
  <input
    id="token-input"
    placeholder="Enter your account token"
    bind:value="{token}"
  />
  <button type="submit">Login</button>
</form>

{#if output}
<p>{output}</p>
{/if}
