<script>
  import { Router, Route, Link } from 'svelte-routing';
  import Index from './routes/index.svelte';
  import About from './routes/about.svelte';
  import Dashboard from './routes/dashboard.svelte';
</script>

<nav>
  <Link to="/">Home</Link> |
  <Link to="/dashboard">Dashboard</Link> |
  <Link to="/about">About</Link>
</nav>

<main>
  <Router>
    <Route path="/" component={Index} />
    <Route path="/dashboard" component={Dashboard} />
    <Route path="/about" component={About} />
  </Router>
</main>

<style>
  nav {
    margin: 1em 0;
    text-align: center;
  }
  nav a {
    margin: 0 1em;
    color: #0074d9;
    text-decoration: none;
    font-weight: bold;
  }
  nav a:hover {
    text-decoration: underline;
  }
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }
  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>