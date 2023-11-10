<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""
  let configNvrViewer = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }

  async function getConfig(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    configNvrViewer = await invoke("get_config", { name })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={getConfig}>
    <button type="submit">Get NVR Viewer Setting</button>
  </form>
  <p>{configNvrViewer}</p>
</div>