<script lang="ts">
	import { aiHost, aiPort, serverPort } from "$lib/stores/settings";
</script>

<!-- This page will be used to either select a video or to input a json object -->

<div id="layout">
  <div id="table">
    <span class="header" data-tooltip="Where the AI is accessible.">AI Address</span>
    <span>
      <input type="text" bind:value={$aiHost} />
      :
      <input type="number" bind:value={$aiPort} 
        min="0" max="65535"
        step="1"
      />
    </span>

    <span class="header" data-tooltip="Where the Backend Mock will be hosted.">Backend Mock</span>
    <span>
      <input type="text" value="0.0.0.0" disabled />
      :
      <input type="number" bind:value={$serverPort} 
        min="0" max="65535"
        step="1"
      />
    </span>
  </div>
</div>

<style lang="scss">
  #layout {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  #table {
    display: grid;
    grid-template-columns: min-content min-content;
    row-gap: 10px;
    column-gap: 20px;
    align-items: center;
    
    span {
      white-space: nowrap;

      &.header {
        font-size: medium;
        font-weight: 500;
      }
    }

    input {
      box-sizing: border-box;
      border-radius: 5px;
      font-family: 'Fira Code', monospace;
      padding: 2px;
    }
  }
</style>