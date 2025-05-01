<script>
  // Example testable logic
  export let title = "About Page";
  export let name = "About";
</script>

<main>
  <h1>{title}</h1>
  <h1>Welcome to {name} Page</h1>
  <!-- Add a testable element -->
  <p data-testid="about-description">This is the about page.</p>
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;
  }
</style>