<script>
  import { videoPath } from "$lib/stores/expectation";
  import { open } from '@tauri-apps/api/dialog'
	import { convertFileSrc } from "@tauri-apps/api/tauri";
  
  import { listen } from '@tauri-apps/api/event'
	import { onDestroy } from "svelte";
  import { blockButtons } from "$lib/stores/global"; 

  import { getNotificationsContext } from 'svelte-notifications';
  const { addNotification } = getNotificationsContext();

  const unlisten = [listen("landmarks_extracted", () => {
    addNotification({
      id: new Date().getTime(),
      text: 'Landmarks extracted!',
      type: 'success',
      position: 'bottom-right',
      removeAfter: 5000
    });

    $blockButtons = false;
  }),
  listen("landmarks_extraction_failed", () => {
    addNotification({
      id: new Date().getTime(),
      text: 'Landmarks extraction failed! Try using a different video.',
      type: 'error',
      position: 'bottom-right',
      removeAfter: 5000
    });

    $blockButtons = false;
    $videoPath = "";
  })];

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

  onDestroy(async () => {
    unlisten.forEach(async unlisten => (await unlisten)());
  });
</script>

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
{/if}

<style>
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
</style>