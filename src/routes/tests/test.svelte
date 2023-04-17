<script lang="ts">
	import Button from '$lib/button.svelte';
  import type { Test } from '$lib/stores/tests';
	import DeleteIcon from '$lib/icons/delete-icon.svelte';
	import PlayIcon from '$lib/icons/play-icon.svelte';
  import { runTest } from '$lib/socket';
	import { aiHost, aiPort } from '$lib/stores/settings';
	import HaltIcon from '$lib/icons/halt-icon.svelte';
	import SpinnerIcon from '$lib/icons/spinner-icon.svelte';
  import { testPaths } from '$lib/stores/tests';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { blockButtons } from '$lib/stores/global';
  
  export let tf: Test;
  export let cancelRunningTest: (() => void) | undefined;
  export let canRunTests: boolean;
</script>

<div class="testCase">
  <div class="header">
    <p class="title">{tf.name}</p>
    <Button
      style="margin: 0; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;"
      onClick={() => {
        if (tf.videoElem && tf.liveFeedbackElem && tf.persistentFeedbackElem) {
          $blockButtons = true;
          tf.running = true;
          tf.token = {};
          cancelRunningTest = () => {
            if (tf.token && tf.token.cancel)
              tf.token.cancel();
          }
          runTest(
            tf.videoElem,
            tf.liveFeedbackElem,
            tf.persistentFeedbackElem,
            $aiHost,
            $aiPort,
            10,
            tf.token
          ).then(() => {
            $blockButtons = false;
            tf.running = false;
          });
        }
      }}
      type="positive"
      disabled={!canRunTests || $blockButtons}
    >
      {#if tf.running}  
        <SpinnerIcon color="white" />
      {:else}
        <PlayIcon color="white" size={15}/>
      {/if}
    </Button>
    {#if !$blockButtons}
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
      src={convertFileSrc(tf.path)}
      crossOrigin="anonymous"
      bind:this={tf.videoElem}
    />
    <textarea readonly bind:this={tf.liveFeedbackElem} placeholder="Live Feedback"></textarea>
    <textarea readonly bind:this={tf.persistentFeedbackElem} placeholder="Persistent Feedback"></textarea>
  </div>
</div>

<style lang="scss">
  .testCase {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px;
    border: 1px solid #e1e1e1;
    border-radius: 5px;

    .header {
      display: flex;
      align-items: center;
      gap: 10px;

      .title {
        margin: 0;
        margin-right: auto;
        font-size: 1.2rem;
        font-weight: 500;
        color: #333;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        flex-shrink: 1;

        @media (prefers-color-scheme: dark) {
          color: #fff;
        }
      }
    }

    .content {
      display: grid;
      grid-template-columns: 1fr 1fr 1fr;
      gap: 10px;

      * {
        border-radius: 5px;
      }

      textarea {
        border: 1px solid #e1e1e1;
        padding: 10px;
        resize: none;
        overflow: auto;
        font-family: 'Fira Code', monospace;
        font-size: 0.8rem;
      }

      video {
        width: 100%;
        height: 100%;
        border-radius: 5px;
        max-height: calc(100vw * 0.5625 * 0.33);
      }
    }
  }

  
</style>