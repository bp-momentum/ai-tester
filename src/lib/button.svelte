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

<style  lang="scss">
  .button {
    background-color: #f1f1f1;
    border-radius: 5px;
    padding: 10px;
    margin: 10px;
    cursor: pointer;
    display: inline-block;
    background-color: cornflowerblue;
    color: white;
    border: 1px solid #404040;

    @media (prefers-color-scheme: dark) {
      border: 1px solid #e1e1e1;
    }

    &.disabled {
      cursor: not-allowed;
      opacity: 0.5;
    }

    &.positive {
      background-color: #4caf50;
    }

    &.negative {
      background-color: #f44336;
    }
  }
</style>