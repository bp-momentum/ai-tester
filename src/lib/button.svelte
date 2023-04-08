<script lang="ts">
  export let onClick: (e?: MouseEvent) => void;
  export let disabled: boolean = false;
  export let type: "default"|"positive"|"negative" = "default";
  export let style: string | undefined = undefined;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div 
  class="button"
  class:disabled
  class:positive={type === "positive"}
  class:negative={type === "negative"}
  on:click={
    (e) => {
      if (!disabled) {
        onClick(e);
      }
    }
  } 
  style={style}
>
  <slot>
    Button
  </slot>
</div>

<style>
  .button {
    background-color: #f1f1f1;
    border: 1px solid #e1e1e1;
    border-radius: 5px;
    padding: 10px;
    margin: 10px;
    cursor: pointer;
    display: inline-block;
    background-color: cornflowerblue;
    color: white;
  }

  .button.disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .button.positive {
    background-color: #4caf50;
  }

  .button.negative {
    background-color: #f44336;
  }
</style>