<script lang="ts">
	import { onDestroy } from "svelte";
	import Button from "$lib/button.svelte";
  import { listen } from '@tauri-apps/api/event'
  import { open } from '@tauri-apps/api/dialog'
  import { inputType, jsonObject, videoPath, jsonValid } from "$lib/stores/expectation";
  import { getNotificationsContext } from 'svelte-notifications';
	import { convertFileSrc } from "@tauri-apps/api/tauri";

  const { addNotification } = getNotificationsContext();

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

  const unlisten = listen('tauri://file-drop', async (event) => {
    if (!event.payload || !((event.payload as Array<string>).length > 0))
      return;
    const payload: Array<string> = event.payload as Array<string>;
    if (payload.length !== 1) {
      addNotification({
        id: new Date().getTime(),
        text: 'You can only drop one file at a time',
        type: 'error',
        position: 'bottom-right',
        removeAfter: 5000
      });
      return;
    }
    let path = payload[0];
    // check file extension
    const allowedExtensions = ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'wmv', 'mpg', 'mpeg'];
    const extension = path.split('.').pop();
    if (!extension || !allowedExtensions.includes(extension)) {
      addNotification({
        id: new Date().getTime(),
        text: 'You can only drop video files',
        type: 'error',
        position: 'bottom-right',
        removeAfter: 5000
      });
      return;
    }
    $videoPath = payload[0];
  });

  onDestroy(async () => {
    (await unlisten)();
  });
</script>

<!-- This page will be used to either select a video or to input a json object -->

<div id="layout">
  {#if $videoPath !== "" || $jsonObject !== ""}
    <div id="buttonContainer">
      <Button 
        onClick={() => {$videoPath = ""; $jsonObject = ""}}
        style="margin: 0;"
        type="negative"
      >
        Reset Input
      </Button>
      <Button 
        onClick={() => {
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
    <!-- svelte-ignore a11y-click-events-have-key-events -->
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
        src={convertFileSrc($videoPath)}
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
      onClick={() => {
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

<style lang="scss">
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
    text-align: center;
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

    &:not(.valid) {
      color: #f00;
    }
  }
</style>