<script>
  // Example testable logic
  export let title = "About Page";
  export let name = "About";
</script>

<main>
  <h1>{title}</h1>
  <h1>Welcome to {name} Page</h1>
  <p data-testid="about-description">This is the about page.</p>
  <section>
    <h2>API Dokumentasi</h2>
    <ul>
      <li><a href="/api/docs" target="_blank">Lihat dokumentasi endpoint backend</a></li>
      <li><a href="https://github.com/onaema/solana-memecoin-bot" target="_blank">Repo GitHub</a></li>
    </ul>
  </section>
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;
  }
</style>