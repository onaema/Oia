<script>
  import { onMount } from 'svelte';
  import { fetchHelloMessage } from '../utils/tauri';

  let message = '';

  onMount(async () => {
    try {
      const response = await fetchHelloMessage();
      message = response.message;
    } catch (error) {
      message = 'Failed to fetch message from backend';
    }
  });
</script>

<main>
  <h1>{message || 'Loading...'}</h1>
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;
  }
</style>