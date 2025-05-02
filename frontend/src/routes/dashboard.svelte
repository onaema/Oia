<script>
  import { onMount, afterUpdate } from 'svelte';
  import { fetchDashboard, fetchTrades } from '../utils/tauri';
  import Chart from 'chart.js/auto';
  let dashboard = null;
  let trades = [];
  let errorMsg = '';
  let chart;
  let chartCanvas;
  let loadingDashboard = false;
  let loadingTrades = false;

  let wsPrice = null;
  let wsConnected = false;
  let wsError = '';
  let livePrice = null;
  let liveSymbol = '';
  let liveTs = '';

  function renderChart() {
    if (chart) chart.destroy();
    if (dashboard && chartCanvas) {
      chart = new Chart(chartCanvas, {
        type: 'bar',
        data: {
          labels: ['Profit', 'Loss'],
          datasets: [{
            label: 'Statistik',
            data: [dashboard.profit, dashboard.loss],
            backgroundColor: ['#2ecc40', '#ff4136']
          }]
        },
        options: { responsive: true, plugins: { legend: { display: false } } }
      });
    }
  }

  async function loadDashboard() {
    loadingDashboard = true;
    try {
      dashboard = await fetchDashboard();
    } catch (e) {
      dashboard = null;
    }
    loadingDashboard = false;
  }

  async function loadTrades() {
    loadingTrades = true;
    try {
      trades = await fetchTrades();
    } catch (e) {
      errorMsg = 'Gagal memuat data dashboard';
    }
    loadingTrades = false;
  }

  function connectWS() {
    wsError = '';
    const ws = new WebSocket('ws://127.0.0.1:3030/api/ws/price');
    ws.onopen = () => { wsConnected = true; };
    ws.onclose = () => { wsConnected = false; };
    ws.onerror = (e) => { wsError = 'WebSocket error'; };
    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        livePrice = data.price;
        liveSymbol = data.symbol;
        liveTs = data.ts;
      } catch (e) { wsError = 'Parse error'; }
    };
    wsPrice = ws;
  }

  onMount(async () => {
    await loadDashboard();
    await loadTrades();
    connectWS();
  });

  afterUpdate(renderChart);
</script>

<main>
  <h1>Dashboard Trading</h1>
  {#if errorMsg}
    <p style="color:red">{errorMsg}</p>
  {/if}
  <section>
    <h2>Statistik</h2>
    {#if loadingDashboard}
      <p>Memuat statistik...</p>
    {:else if dashboard}
      <ul style="list-style:none">
        <li><b>Total Trades:</b> {dashboard.total_trades}</li>
        <li><b>Profit:</b> {dashboard.profit}</li>
        <li><b>Loss:</b> {dashboard.loss}</li>
      </ul>
      <canvas bind:this={chartCanvas} width="300" height="150"></canvas>
    {:else}
      <p>Gagal memuat statistik</p>
    {/if}
  </section>
  <section>
    <h2>Riwayat Trade</h2>
    {#if loadingTrades}
      <p>Memuat riwayat trade...</p>
    {:else if trades.length > 0}
      <table border="1" style="margin:auto">
        <thead>
          <tr>
            <th>ID</th><th>Symbol</th><th>Amount</th><th>Status</th>
          </tr>
        </thead>
        <tbody>
          {#each trades as t}
            <tr>
              <td>{t.id}</td>
              <td>{t.symbol}</td>
              <td>{t.amount}</td>
              <td>{t.status}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p>Tidak ada trade.</p>
    {/if}
  </section>
  <section>
    <h2>Harga Live (WebSocket)</h2>
    {#if wsConnected}
      <p><b>{liveSymbol}</b>: {livePrice} <small>{liveTs}</small></p>
    {:else if wsError}
      <p style="color:red">{wsError}</p>
    {:else}
      <p>Menghubungkan WebSocket...</p>
    {/if}
  </section>
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;
  }
</style>
