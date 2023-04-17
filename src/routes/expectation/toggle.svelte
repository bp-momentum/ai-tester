<script lang="ts">
	import Button from "$lib/button.svelte";
  import { inputType, jsonObject, videoPath } from "$lib/stores/expectation";
  import { blockButtons } from "$lib/stores/global";
</script>

<div id="buttonContainer">
  <Button 
    onClick={() => {
      $jsonObject = "";
      $videoPath = "";
    }}
    style="margin: 0;"
    type="negative"
    disabled={$blockButtons}
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
    disabled={$blockButtons}
  >
    {#if $inputType === "video"}
    Rather input a JSON object
    {:else}
    Rather select a video
    {/if}
  </Button>
</div>

<style>
  #buttonContainer {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin: 0 0 10px 0;
  }
</style>