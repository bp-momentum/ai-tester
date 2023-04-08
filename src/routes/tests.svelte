<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { testPaths, type Test, running } from '$lib/stores/tests';
	import Button from '$lib/button.svelte';
  import { getNotificationsContext } from 'svelte-notifications';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import DeleteIcon from '$lib/icons/delete-icon.svelte';
	import PlayIcon from '$lib/icons/play-icon.svelte';
  import { runTest } from '$lib/socket';
	import { aiHost, aiPort } from '$lib/stores/settings';
	import HaltIcon from '$lib/icons/halt-icon.svelte';
	import SpinnerIcon from '$lib/icons/spinner-icon.svelte';

  const { addNotification } = getNotificationsContext();

  const rsplit = (inp: string, sep: string, maxsplit: number) => {
    var split = inp.split(sep);
    return maxsplit ? [ split.slice(0, -maxsplit).join(sep) ].concat(split.slice(-maxsplit)) : split;
  }

  let canRunTests: boolean = false;
  invoke('can_run_tests')
    .then(can => {typeof can === "boolean" && (canRunTests = can)});

  const openVideos = async () => {
    const filePaths = await open({
      multiple: true,
      directory: false,
      filters: [
        {
          name: 'Video',
          extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'wmv', 'mpg', 'mpeg']
        }
      ]
    });
    if (filePaths && typeof filePaths === 'object' && filePaths.length > 0) {
      filePaths.forEach(element => {
        if (!$testPaths.filter(tf => tf.path === element).length)
          $testPaths.push({
            name: rsplit(element.replaceAll("\\", "/"), '/', 1)[1],
            path: element,
            running: false,
          });
      });
      testPaths.update(tests => tests);
    }
  };

  const unlisten = listen('tauri://file-drop', async (event) => {
    if (!event.payload || !((event.payload as Array<string>).length > 0))
      return;
    const payload: Array<string> = event.payload as Array<string>;
    // check file extension
    const allowedExtensions = ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'wmv', 'mpg', 'mpeg'];
    let illegalFiles = false;
    let legalFiles: Array<string> = [];
    payload.forEach(path => {
      const extension = path.split('.').pop();
      if (!extension || !allowedExtensions.includes(extension)) {
        illegalFiles = true;
        return;
      }
      legalFiles.push(path);
    });
    if (illegalFiles) {
      addNotification({
        id: new Date().getTime(),
        text: 'Some files were not added because they are not video files',
        type: 'warning',
        position: 'bottom-right',
        removeAfter: 5000
      });
    }
    if (legalFiles.length > 0) {
      legalFiles.forEach(element => {
        if (!$testPaths.filter(tf => tf.path === element).length)
        $testPaths.push({
            name: rsplit(element.replaceAll("\\", "/"), '/', 1)[1],
            path: element,
            running: false,
          });
      });
    }
    testPaths.update(tests => tests);
  });

  onDestroy(async () => {
    (await unlisten)();
  });
</script>

<div id="list">
  {#each $testPaths as tf}
    <div class="testCase">
      <div class="header">
        <p class="title">{tf.name}</p>
        <Button
          style="margin: 0; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;"
          onClick={() => {
            if (tf.videoElem && tf.liveFeedbackElem && tf.persistentFeedbackElem) {
              $running = true;
              tf.running = true;
              tf.token = {};
              runTest(
                tf.videoElem,
                tf.liveFeedbackElem,
                tf.persistentFeedbackElem,
                $aiHost,
                $aiPort,
                10,
                tf.token
              ).then(() => {
                $running = false;
                tf.running = false;
              });
            }
          }}
          type="positive"
          disabled={!canRunTests || $running}
        >
          {#if tf.running}  
            <SpinnerIcon color="white" />
          {:else}
            <PlayIcon color="white" size={15}/>
          {/if}
        </Button>
        {#if !$running}
        <Button 
          style="margin: 0; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;"
          onClick={() => $testPaths = $testPaths.filter(t => t !== tf)}
          type="negative"
        >
          <DeleteIcon color="white"/>
        </Button>
        {:else}
        <Button 
          style="margin: 0; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;"
          onClick={() => {
            if (tf.videoElem && tf.liveFeedbackElem && tf.persistentFeedbackElem) {
              if (tf.token && tf.token.cancel)
                tf.token.cancel();
            }
          }}
          type="negative"
          disabled={!tf.running}
        >
          <HaltIcon color="white" size={30}/>
        </Button>
        {/if}
      </div>
      <div class="content">
        <!-- svelte-ignore a11y-media-has-caption -->
        <video 
          id="video"
          controls
          controlslist="nodownload nofullscreen noremoteplayback noplaybackrate"
          disablepictureinpicture
          disableremoteplayback
          src={"https://video.localhost/" + btoa(tf.path)}
          crossOrigin="anonymous"
          bind:this={tf.videoElem}
        />
        <textarea readonly bind:this={tf.liveFeedbackElem} placeholder="Live Feedback"></textarea>
        <textarea readonly bind:this={tf.persistentFeedbackElem} placeholder="Persistent Feedback"></textarea>
      </div>
    </div>
  {/each}

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div 
    id="addButton" 
    on:click={openVideos}
  >
    <h2>Add Tests</h2>
    <p>Click here to select more test files<br>
    or drag and drop a video file here</p>
  </div>
</div>

<style>
  #list {
    display: flex;
    flex-direction: column;
    row-gap: 10px;
  }

  .testCase {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px;
    border: 1px solid #e1e1e1;
    border-radius: 5px;
  }

  .testCase .header {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  
  .testCase .header .title {
    margin: 0;
    margin-right: auto;
    font-size: 1.2rem;
    font-weight: 500;
    color: #333;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex-shrink: 1;
  }

  .testCase .content {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 10px;
  }

  .testCase .content * {
    border-radius: 5px;
  }

  .testCase .content textarea {
    border: 1px solid #e1e1e1;
    padding: 10px;
    resize: none;
    overflow: auto;
    font-family: 'Fira Code', monospace;
    font-size: 0.8rem;
    color: #333;
  }

  .testCase .content video {
    width: 100%;
    height: 100%;
    border-radius: 5px;
    max-height: calc(100vw * 0.5625 * 0.33);
  }

  #addButton {
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
</style>