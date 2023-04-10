<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { testPaths } from '$lib/stores/tests';
  import { getNotificationsContext } from 'svelte-notifications';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Test from './test.svelte';

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

  let cancelRunningTest: (() => void) | undefined = undefined;

  onDestroy(async () => {
    (await unlisten)();
    cancelRunningTest && cancelRunningTest();
  });
</script>

<div id="list">
  {#each $testPaths as tf}
    <Test {tf} {cancelRunningTest} {canRunTests} />
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

<style lang="scss">
  #list {
    display: flex;
    flex-direction: column;
    row-gap: 10px;
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