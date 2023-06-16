<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let output = false;
  let outputUrl = false;

  function fallbackCopyTextToClipboard(text) {
    var textArea = document.createElement("textarea");
    textArea.value = text;

    // Avoid scrolling to bottom
    textArea.style.top = "0";
    textArea.style.left = "0";
    textArea.style.position = "fixed";

    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();

    try {
      var successful = document.execCommand("copy");
      var msg = successful ? "successful" : "unsuccessful";
      console.log("Fallback: Copying text command was " + msg);
    } catch (err) {
      console.error("Fallback: Oops, unable to copy", err);
    }

    document.body.removeChild(textArea);
  }
  function copyTextToClipboard(text) {
    if (!navigator.clipboard) {
      fallbackCopyTextToClipboard(text);
      return;
    }
    navigator.clipboard.writeText(text).then(
      function () {
        console.log("Async: Copying to clipboard was successful!");
      },
      function (err) {
        console.error("Async: Could not copy text: ", err);
      }
    );
  }

  async function handlerFn() {
    let res = await invoke("nordvpn_login", {});
    output = res;
    let regex =
      /Continue in the browser: (https:\/\/api\.nordvpn\.com\/v1\/users\/oauth\/login-redirect\?attempt=([A-Za-z0-9]+(-[A-Za-z0-9]+)+))/i;
    let x = regex.exec(output);
    if (x && x.length) {
      outputUrl = x[1];
    }
  }

  function CopyUrl() {
    copyTextToClipboard(outputUrl);
  }
</script>

<div class="row">
  <button on:click="{handlerFn}">Login with NordVPN Account</button>
</div>

{#if output}
<p>{output}</p>
{/if} {#if outputUrl}
<div class="row">
  <button on:click="{CopyUrl}">COPY URL : {outputUrl}</button>
</div>
<p>Open the above copied url on your browser and continue login.</p>
{/if}
