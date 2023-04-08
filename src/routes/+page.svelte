<script>
  import Expectation from './expectation.svelte';
  import Tests from './tests.svelte';
  import Notifications from 'svelte-notifications';
	import Settings from './settings.svelte';
	import { running } from '$lib/stores/tests';

  export let menu = 0;
</script>

<Notifications>
  <div id="navbar">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div id="navbar-item" class:disabled={$running} on:click={() => {
      if (!$running) menu = 0
    }}>
      <h2 class:selected={menu === 0}>Expectation</h2>
    </div> 
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div id="navbar-item" class:disabled={$running} on:click={() => {
      if (!$running) menu = 1
    }}>
      <h2 class:selected={menu === 1}>Tests</h2>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div id="navbar-item" class:disabled={$running} on:click={() => {
      if (!$running) menu = 2
    }}>
      <h2 class:selected={menu === 2}>Settings</h2>
    </div>
  </div>

  <div id="content">
    {#if menu === 0}
      <Expectation />
    {:else if menu === 1}
      <Tests />
    {:else if menu === 2}
      <Settings />
    {:else}
      <!-- use an empty slot to force a 404 -->
      <slot /> 
    {/if}
  </div>
</Notifications>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap');
  @import url('https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&display=swap');

  :global(*) {
    font-family: 'Fira Sans', sans-serif;
  }

  :global(html, body) {
    margin: 0;
  }

  #navbar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    background-color: #f1f1f1;
    padding: 10px;
  }

  #navbar-item {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 100%;
    padding: 10px;
    cursor: pointer;
    border-radius: 5px;
  }

  #navbar-item.disabled {
    cursor: not-allowed;
  }

  #navbar-item.disabled:hover {
    background-color: unset;
  }

  #navbar-item:hover {
    background-color: #e1e1e1;
  }

  #navbar-item h2 {
    margin: 0;
  }

  #navbar-item h2.selected {
    text-decoration: underline;
  }

  #content {
    padding: 10px;
    overflow-y: auto;
  }
</style>