<script lang="ts">
	import { onDestroy } from "svelte";
	import Button from "$lib/button.svelte";
  import { listen } from '@tauri-apps/api/event'
  import { inputType, jsonObject, videoPath, jsonValid } from "$lib/stores/expectation";
  import { getNotificationsContext } from 'svelte-notifications';
	import Toggle from "./toggle.svelte";
	import VideoInput from "./videoInput.svelte";

  const { addNotification } = getNotificationsContext();

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
    <Toggle />
  {/if}

  {#if $inputType === "video"}
    <VideoInput />
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