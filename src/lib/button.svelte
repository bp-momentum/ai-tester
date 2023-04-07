<script lang="ts">
  // accept custom style
  export let style: string | undefined = undefined;
</script>

<div class="button" on:click style={style}>
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
</style>