<script lang="ts">
  import { FilesetResolver, PoseLandmarker } from "@mediapipe/tasks-vision";
  import type { NormalizedLandmark } from "@mediapipe/tasks-vision";

  import { videoJson, videoJustSet, videoPath, videoValid } from "$lib/stores/expectation";
  import { open } from '@tauri-apps/api/dialog'
	import { convertFileSrc } from "@tauri-apps/api/tauri";
  
  import { blockButtons } from "$lib/stores/global"; 

  import { getNotificationsContext } from 'svelte-notifications';
	import { onDestroy } from "svelte";

  const { addNotification } = getNotificationsContext();

  const unlisten = videoValid.subscribe((valid) => {
    if (valid) {
      addNotification({
        id: new Date().getTime(),
        text: 'Video processed',
        type: 'success',
        position: 'bottom-right',
        removeAfter: 5000
      });
    } else if (valid === false) {
      $videoPath = "";
      addNotification({
        id: new Date().getTime(),
        text: 'Could not process video',
        type: 'error',
        position: 'bottom-right',
        removeAfter: 5000
      });
    }
    $videoValid = undefined;
    $blockButtons = false;
  });

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

  const debounce = (fn: () => void, ms: number) => {
    let timeoutId: number;
    return () => {
      clearTimeout(timeoutId);
      timeoutId = setTimeout(fn, ms);
    };
  };

  const debouncedNoLandmarkWarning = debounce(() => {
    addNotification({
      id: new Date().getTime(),
      text: 'No landmarks found',
      type: 'warning',
      position: 'bottom-right',
      removeAfter: 5000
    });
  }, 1000);

  const processVideo = async (e: Event) => {
    if (!$videoJustSet) return;
    $videoJustSet = false;

    const video = (e.target as HTMLVideoElement);
    if (!video) return;

    $blockButtons = true;

    const vision = await FilesetResolver.forVisionTasks(
      "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@latest/wasm"
    );
    const poseLandmarker = await PoseLandmarker.createFromOptions(
      vision,
    {
      baseOptions: {
        modelAssetPath: "https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_lite/float16/1/pose_landmarker_lite.task",
        delegate: "GPU"
      },
      runningMode: "IMAGE",
      minPoseDetectionConfidence: 0.5,
      minPosePresenceConfidence: 0.5,
      minTrackingConfidence: 0.5,
    });

    video.pause();
    video.currentTime = 0;
    // step through the video in 10 fps steps
    const fps = 10;
    const step = Math.round(video.duration * fps);

    const landMarkList: NormalizedLandmark[][] = new Array<NormalizedLandmark[]>(step);

    let sum = 0;

    for (let i = 0; i < step; i++) {
      const canvas = document.createElement('canvas');
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      const context = canvas.getContext('2d');
      if (!context) {
        addNotification({
          id: new Date().getTime(),
          text: 'Could not process video',
          type: 'error',
          position: 'bottom-right',
          removeAfter: 5000
        });

        $blockButtons = false;
        $videoPath = "";
        return;
      }

      video.currentTime = i / fps;
      context.drawImage(video, 0, 0, canvas.width, canvas.height);
      poseLandmarker.detect(canvas, (result) => {
        if (result.landmarks[0] === undefined) {
          debouncedNoLandmarkWarning();
        }

        landMarkList[i] = result.landmarks[0];
        sum += 1;
      });
    }

    // wait for all frames to be processed
    while (sum < step) {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
    
    poseLandmarker.close();

    $videoJson = landMarkList;
  }

  onDestroy(() => {
    unlisten();
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
    crossOrigin="anonymous"
    on:loadeddata={processVideo}
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