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

  // Example testable logic
  export let welcomeMessage = "Welcome to the Homepage!";
</script>

<main>
  <h1>{message || 'Loading...'}</h1>
  <h1>{welcomeMessage}</h1>
  <!-- Add a testable element -->
  <p data-testid="homepage-description">This is the homepage.</p>
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;
  }
</style>