<script>
  import { open } from '@tauri-apps/api/dialog';
  
  const openFile = async () => {
    const filePaths = await open();
    console.log(filePaths);
  };
</script>

<h1>Welcome to TESTS</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>