<script lang="ts">
	import Button from "$lib/button.svelte";
  import { listen } from '@tauri-apps/api/event'
  import { open } from '@tauri-apps/api/dialog'
  import { inputType, jsonObject, videoPath, jsonValid } from "./stores/expectation";

  const openVideo = async () => {
    const filePaths = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'Video',
          extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'wmv', 'mpg', 'mpeg']
        }
      ]
    });
    if (filePaths && typeof filePaths === 'string')
      $videoPath = filePaths;
  };

  listen('tauri://file-drop', (event) => {
    if (!event.payload || !((event.payload as Array<string>).length > 0))
      return;
    const payload: Array<string> = event.payload as Array<string>;
    $videoPath = payload[0];
  });

</script>

<!-- This page will be used to either select a video or to input a json object -->

<div id="layout">
  {#if $videoPath !== "" || $jsonObject !== ""}
    <div id="buttonContainer">
      <Button 
        on:click={() => {$videoPath = ""; $jsonObject = ""}}
        style="background: #b00; margin: 0;"
      >
        Reset Input
      </Button>
      <Button 
        on:click={() => {
          $jsonObject = "";
          $videoPath = "";
          $inputType = ($inputType === "video" ? "string" : "video");
        }}
        style="margin: 0;"
      >
        {#if $inputType === "video"}
        Rather input a JSON object
        {:else}
        Rather select a video
        {/if}
      </Button>
    </div>
  {/if}

  {#if $inputType === "video"}
    {#if $videoPath === ""}
    <div
      id="videoInput"
      on:click={openVideo}
    >
      <h2>Expectation Video</h2>
      <p>Click here to select a video or<br>
      drag and drop a video file here</p>
      </div>
    {:else}
      <!-- svelte-ignore a11y-media-has-caption -->
      <video 
        id="video"
        controls
        controlslist="nodownload nofullscreen noremoteplayback noplaybackrate"
        disablepictureinpicture
        disableremoteplayback
        src={"https://video.localhost/" + btoa($videoPath)}
      />
      <p>
        <b>Warning:</b> Not yet implemented!
      </p>
    {/if}
  {:else}
    <textarea 
      placeholder="Input a JSON object here"
      id="textInput"
      class:valid={$jsonValid}
      bind:value={$jsonObject}
    ></textarea>
  {/if}

  {#if $videoPath === "" && $jsonObject === ""}
    <Button 
      on:click={() => {
        $jsonObject = "";
        $videoPath = "";
        $inputType = ($inputType === "video" ? "string" : "video");
      }}
    >
      {#if $inputType === "video"}
      Rather input a JSON object
      {:else}
      Rather select a Video
      {/if}
    </Button>
  {/if}

</div>

<style>
  #layout {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    position: relative;
  }

  #buttonContainer {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin: 0 0 10px 0;
  }

  #videoInput {
    width: 100%;
    box-sizing: border-box;
    border: 2px dashed #e1e1e1;
    border-radius: 5px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #959595;
    height: 180px;
    cursor: pointer;
  }

  #video {
    max-width: 100%;
    border-radius: 5px;
  }

  #textInput {
    width: 100%;
    box-sizing: border-box;
    height: 180px;
    resize: none;
    padding: 5px;
    border-radius: 5px;
    font-size: medium;
    font-family: 'Fira Code', monospace;
  }

  #textInput:not(.valid) {
    color: #f00;
  }
</style>