<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { testPaths } from './stores/tests';
	import Button from '$lib/button.svelte';
  import { getNotificationsContext } from 'svelte-notifications';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';

  const { addNotification } = getNotificationsContext();

  String.prototype.rsplit = function(sep: string, maxsplit: number) {
    var split = this.split(sep);
    return maxsplit ? [ split.slice(0, -maxsplit).join(sep) ].concat(split.slice(-maxsplit)) : split;
  }

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
    if (filePaths && typeof filePaths === 'object' && filePaths.length > 0)
      $testPaths = [...new Set($testPaths.concat(filePaths))];
    console.log($testPaths)
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
    if (legalFiles.length > 0)
      $testPaths = [...new Set($testPaths.concat(legalFiles))];
  });

  onDestroy(async () => {
    (await unlisten)();
  });
</script>

<div id="list">
  {#each $testPaths as tf}
    <div class="testCase">
      <div class="header">
        <p class="title">{tf.replaceAll("\\", "/").rsplit("/", 1)[1]}</p>
        <Button 
          style="background: #0b0; margin: 0; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;"
          on:click={() => addNotification({
            id: new Date().getTime(),
            text: 'Not implemented yet',
            type: 'warning',
            position: 'bottom-right',
            removeAfter: 5000
          })}
        >
          ▶
        </Button>
        <Button 
          style="background: #b00; margin: 0; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;"
          on:click={() => $testPaths = $testPaths.filter(t => t !== tf)}
        >
          🗑️
        </Button>
      </div>
      <div class="content">
        <video 
          id="video"
          controls
          controlslist="nodownload nofullscreen noremoteplayback noplaybackrate"
          disablepictureinpicture
          disableremoteplayback
          src={"https://video.localhost/" + btoa(tf)}
        />
        <textarea readonly>Live Feedback</textarea>
        <textarea readonly>Persistent Feedback</textarea>
      </div>
    </div>
  {/each}

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