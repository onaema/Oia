<script>
  import { onMount } from 'svelte';
  import { fetchHelloMessage } from '../utils/tauri';
  import {
    fetchTrades,
    fetchWalletBalance,
    postTrade,
    cancelTrade,
    updateTradeStatus,
    fetchDashboard
  } from '../utils/tauri';

  let message = '';
  let trades = [];
  let balance = null;
  let tradeForm = { symbol: '', amount: 0, side: 'buy' };
  let tradeResult = '';
  let tradeStatusUpdate = { id: '', status: '' };
  let cancelId = '';
  let errorMsg = '';
  let dashboard = null;

  let showNotif = false;
  let notifMsg = '';
  let notifType = '';

  let loadingTrades = false;
  let loadingBalance = false;
  let loadingDashboard = false;

  $: tradeFormValid = tradeForm.symbol.trim().match(/^[A-Z0-9/]+$/) && tradeForm.amount > 0 && (tradeForm.side === 'buy' || tradeForm.side === 'sell');

  function showNotification(msg, type = 'info') {
    notifMsg = msg;
    notifType = type;
    showNotif = true;
    setTimeout(() => showNotif = false, 3000);
  }

  async function loadTrades() {
    loadingTrades = true;
    try {
      trades = await fetchTrades();
    } catch (e) {
      errorMsg = 'Gagal mengambil riwayat trade';
    }
    loadingTrades = false;
  }

  async function loadBalance() {
    loadingBalance = true;
    try {
      const res = await fetchWalletBalance();
      balance = res.balance;
    } catch (e) {
      errorMsg = 'Gagal mengambil saldo wallet';
    }
    loadingBalance = false;
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

  async function submitTrade() {
    tradeResult = '';
    try {
      const res = await postTrade(tradeForm);
      tradeResult = res.message;
      showNotification(res.message, res.success ? 'success' : 'error');
      await loadTrades();
      await loadDashboard();
    } catch (e) {
      tradeResult = 'Trade gagal';
      showNotification('Trade gagal', 'error');
    }
  }

  async function submitCancel() {
    try {
      const res = await cancelTrade(Number(cancelId));
      tradeResult = res.message;
      showNotification(res.message, res.success ? 'success' : 'error');
      await loadTrades();
      await loadDashboard();
    } catch (e) {
      tradeResult = 'Cancel gagal';
      showNotification('Cancel gagal', 'error');
    }
  }

  async function submitUpdateStatus() {
    try {
      const res = await updateTradeStatus(Number(tradeStatusUpdate.id), tradeStatusUpdate.status);
      tradeResult = res.message;
      showNotification(res.message, res.success ? 'success' : 'error');
      await loadTrades();
      await loadDashboard();
    } catch (e) {
      tradeResult = 'Update status gagal';
      showNotification('Update status gagal', 'error');
    }
  }

  onMount(async () => {
    try {
      const response = await fetchHelloMessage();
      message = response.message;
    } catch (error) {
      message = 'Failed to fetch message from backend';
    }
    await loadTrades();
    await loadBalance();
    await loadDashboard();
  });

  // Example testable logic
  export let welcomeMessage = "Welcome to the Homepage!";
</script>

<main>
  {#if showNotif}
    <div class="notif {notifType}">{notifMsg}</div>
  {/if}

  <h1>{message || 'Loading...'}</h1>
  <h1>{welcomeMessage}</h1>
  <p data-testid="homepage-description">This is the homepage.</p>

  <section>
    <h2>Saldo Wallet</h2>
    {#if loadingBalance}
      <p>Memuat saldo...</p>
    {:else}
      <p>{balance !== null ? `${balance} lamports` : 'Gagal memuat saldo'}</p>
    {/if}
  </section>

  <section>
    <h2>Dashboard Statistik</h2>
    {#if loadingDashboard}
      <p>Memuat statistik...</p>
    {:else if dashboard}
      <ul style="list-style:none">
        <li><b>Total Trades:</b> {dashboard.total_trades}</li>
        <li><b>Profit:</b> {dashboard.profit}</li>
        <li><b>Loss:</b> {dashboard.loss}</li>
      </ul>
    {:else}
      <p>Gagal memuat statistik</p>
    {/if}
  </section>

  <section>
    <h2>Form Trade</h2>
    <form on:submit|preventDefault={submitTrade}>
      <input placeholder="Symbol" bind:value={tradeForm.symbol} required pattern="[A-Z0-9/]+" />
      <input type="number" placeholder="Amount" bind:value={tradeForm.amount} min="0.0001" step="any" required />
      <select bind:value={tradeForm.side}>
        <option value="buy">Buy</option>
        <option value="sell">Sell</option>
      </select>
      <button type="submit" disabled={!tradeFormValid}>Trade</button>
    </form>
    {#if tradeResult}
      <p style="color: {tradeResult.includes('success') ? 'green' : 'red'}">{tradeResult}</p>
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
    <h2>Cancel Trade</h2>
    <form on:submit|preventDefault={submitCancel}>
      <input type="number" placeholder="Trade ID" bind:value={cancelId} required />
      <button type="submit">Cancel</button>
    </form>
    {#if tradeResult && tradeResult.includes('cancel')}
      <p style="color: {tradeResult.includes('success') ? 'green' : 'red'}">{tradeResult}</p>
    {/if}
  </section>

  <section>
    <h2>Update Status Trade</h2>
    <form on:submit|preventDefault={submitUpdateStatus}>
      <input type="number" placeholder="Trade ID" bind:value={tradeStatusUpdate.id} required />
      <input placeholder="Status Baru" bind:value={tradeStatusUpdate.status} required />
      <button type="submit">Update</button>
    </form>
    {#if tradeResult && tradeResult.includes('update')}
      <p style="color: {tradeResult.includes('success') ? 'green' : 'red'}">{tradeResult}</p>
    {/if}
  </section>

  {#if errorMsg}
    <p style="color:red">{errorMsg}</p>
  {/if}
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;
  }
  .notif {
    position: fixed;
    top: 1rem;
    right: 1rem;
    background: #222;
    color: #fff;
    padding: 1em 2em;
    border-radius: 8px;
    z-index: 1000;
    opacity: 0.95;
    font-weight: bold;
    box-shadow: 0 2px 8px #0003;
    transition: background 0.2s;
  }
  .notif.success { background: #2ecc40; }
  .notif.error { background: #ff4136; }
  .notif.info { background: #0074d9; }
</style>