<script lang="ts">

	import Notifications from "svelte-notifications";
  import { running } from "$lib/stores/tests";

  import { page } from '$app/stores';

  let path: String;

  $: path = $page.url.pathname;

</script>

<Notifications>
  <div id="navbar">
    <a 
      id="navbar-item"
      aria-disabled={$running}
      href="/expectation"
      on:click={e => {if ($running) e.preventDefault()}}
    >
      <h2 class:selected={path === "/expectation"}>Expectation</h2>
    </a> 
    <a
      id="navbar-item"
      aria-disabled={$running}
      href="/tests"
      on:click={e => {if ($running) e.preventDefault()}}
    >
      <h2 class:selected={path === "/tests"}>Tests</h2>
    </a>
    <a
      id="navbar-item"
      aria-disabled={$running}
      href="/settings"
      on:click={e => {if ($running) e.preventDefault()}}
    >
      <h2 class:selected={path === "/settings"}>Settings</h2>
    </a>
  </div>

  <div id="content">
    <slot />
  </div>
</Notifications>

<style lang="scss">
  #navbar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    background-color: #f1f1f1;
    padding: 10px;

    @media (prefers-color-scheme: dark) {
      background-color: #262626;
    }
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
    color: unset;
    text-decoration: none;

    &[aria-disabled="true"] {
      cursor: not-allowed;

      &:hover {
        background-color: unset;
      }
    }

    &:hover {
      background-color: #e1e1e1;

      @media (prefers-color-scheme: dark) {
        background-color: #151515;
      }
    }

    h2 {
      margin: 0;

      &.selected {
        text-decoration: underline;
      }
    }
  }

  #content {
    padding: 10px;
  }

  @media (prefers-color-scheme: dark) {
    #navbar-item:hover {
      background-color: #151515;
    }
  }
</style>

<svelte:head>
  <style>
    @import url('https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap');
    @import url('https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&display=swap');

    * {
      font-family: 'Fira Sans', sans-serif;
      color-scheme: light dark;
    }

    html, body {
      margin: 0;
    }

    [data-tooltip] {
      position: relative;
      z-index: 10;
      display: block;
    }

    [data-tooltip]:before,
    [data-tooltip]:after {
      visibility: hidden;
      opacity: 0;
      pointer-events: none;
      transition: .2s ease-out;
      transform: translate(-50%, 5px);
    }

    [data-tooltip]:before {
      position: absolute;
      bottom: 100%;
      left: 50%;
      margin-bottom: 5px;
      padding: 7px;
      min-width: 100%;
      max-width: 300px;
      -webkit-border-radius: 3px;
      -moz-border-radius: 3px;
      border-radius: 3px;
      background-color: #5e6;
      color: #000;
      content: attr(data-tooltip);
      text-align: center;
      font-size: 14px;
      line-height: 1.2;
      transition: .2s ease-out;
    }

    [data-tooltip]:after {
      position: absolute;
      bottom: 100%;
      left: 50%;
      width: 0;
      border-top: 5px solid #5e6;
      border-right: 5px solid transparent;
      border-left: 5px solid transparent;
      content: " ";
      font-size: 0;
      line-height: 0;
    }

    [data-tooltip]:hover:before,
    [data-tooltip]:hover:after {
      visibility: visible;
      opacity: 1;
      transform: translate(-50%, 0)
    }
    [data-tooltip=false]:hover:before,
    [data-tooltip=false]:hover:after {
      visibility: hidden;
      opacity: 0;
    }
  </style>
</svelte:head>