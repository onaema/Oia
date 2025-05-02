<script>
  import { onMount } from 'svelte';
  let config = null;
  let history = [];
  let errorMsg = '';
  let notif = '';
  let form = {
    min_liquidity: 1.0,
    max_buy: 0.1,
    whitelist: '',
    blacklist: '',
    auto_buy: false
  };

  async function loadConfig() {
    try {
      const res = await fetch('/api/sniper/config');
      config = await res.json();
      form.min_liquidity = config.min_liquidity;
      form.max_buy = config.max_buy;
      form.whitelist = config.whitelist.join(',');
      form.blacklist = config.blacklist.join(',');
      form.auto_buy = config.auto_buy;
    } catch (e) { errorMsg = 'Gagal memuat config sniping'; }
  }
  async function loadHistory() {
    try {
      const res = await fetch('/api/sniper/history');
      history = await res.json();
    } catch (e) { errorMsg = 'Gagal memuat history snipes'; }
  }
  async function saveConfig() {
    notif = '';
    try {
      const body = {
        min_liquidity: parseFloat(form.min_liquidity),
        max_buy: parseFloat(form.max_buy),
        whitelist: form.whitelist.split(',').map(s => s.trim()).filter(Boolean),
        blacklist: form.blacklist.split(',').map(s => s.trim()).filter(Boolean),
        auto_buy: !!form.auto_buy
      };
      const res = await fetch('/api/sniper/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
      });
      if ((await res.json()).success) {
        notif = 'Config berhasil disimpan';
        await loadConfig();
      } else {
        notif = 'Gagal menyimpan config';
      }
    } catch (e) { notif = 'Gagal menyimpan config'; }
  }
  onMount(async () => {
    await loadConfig();
    await loadHistory();
    // (Opsional) polling history setiap 10 detik
    setInterval(loadHistory, 10000);
  });
</script>
<main>
  <h1>Sniper Meme Token Dashboard</h1>
  {#if errorMsg}
    <p style="color:red">{errorMsg}</p>
  {/if}
  <section>
    <h2>Status & Pengaturan Sniping</h2>
    {#if config}
      <form on:submit|preventDefault={saveConfig}>
        <label>Min Liquidity: <input type="number" step="any" bind:value={form.min_liquidity} required /></label><br>
        <label>Max Buy: <input type="number" step="any" bind:value={form.max_buy} required /></label><br>
        <label>Whitelist (comma): <input bind:value={form.whitelist} /></label><br>
        <label>Blacklist (comma): <input bind:value={form.blacklist} /></label><br>
        <label><input type="checkbox" bind:checked={form.auto_buy} /> Auto Buy</label><br>
        <button type="submit">Simpan</button>
      </form>
      {#if notif}
        <p style="color:green">{notif}</p>
      {/if}
      <ul style="list-style:none">
        <li><b>Min Liquidity:</b> {config.min_liquidity}</li>
        <li><b>Max Buy:</b> {config.max_buy}</li>
        <li><b>Whitelist:</b> {config.whitelist.join(', ')}</li>
        <li><b>Blacklist:</b> {config.blacklist.join(', ')}</li>
        <li><b>Auto Buy:</b> {config.auto_buy ? 'Aktif' : 'Mati'}</li>
      </ul>
    {:else}
      <p>Memuat config...</p>
    {/if}
  </section>
  <section>
    <h2>History Snipes</h2>
    {#if history.length > 0}
      <table border="1" style="margin:auto">
        <thead>
          <tr>
            <th>Token</th><th>Waktu</th><th>Status</th><th>Tx Hash</th><th>Harga</th>
          </tr>
        </thead>
        <tbody>
          {#each history as h}
            <tr>
              <td>{h.token}</td>
              <td>{h.time}</td>
              <td>{h.status}</td>
              <td>{h.tx_hash}</td>
              <td>{h.price}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p>Belum ada snipes.</p>
    {/if}
  </section>
</main>
<style>
  main { padding: 1rem; text-align: center; }
  table { margin: 1em auto; }
  input[type="number"] { width: 6em; }
</style>
