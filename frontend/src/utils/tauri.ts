const API_URL = 'http://127.0.0.1:3030';
const API_KEY = 'secret'; // Ganti sesuai backend

export async function fetchHelloMessage() {
  try {
    const response = await fetch(`${API_URL}/api/hello`);
    if (!response.ok) {
      throw new Error('Failed to fetch message');
    }
    return await response.json();
  } catch (error) {
    console.error('Error fetching message:', error);
    throw error;
  }
}

export async function fetchTrades() {
  const res = await fetch(`${API_URL}/api/trades`, {
    headers: { 'X-API-KEY': API_KEY }
  });
  if (!res.ok) throw new Error('Failed to fetch trades');
  return await res.json();
}

export async function fetchWalletBalance() {
  const res = await fetch(`${API_URL}/api/wallet/balance`, {
    headers: { 'X-API-KEY': API_KEY }
  });
  if (!res.ok) throw new Error('Failed to fetch wallet balance');
  return await res.json();
}

export async function postTrade({ symbol, amount, side }) {
  const res = await fetch(`${API_URL}/api/trade`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-KEY': API_KEY
    },
    body: JSON.stringify({ symbol, amount, side })
  });
  return await res.json();
}

export async function cancelTrade(id) {
  const res = await fetch(`${API_URL}/api/trade/cancel`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-KEY': API_KEY
    },
    body: JSON.stringify({ id })
  });
  return await res.json();
}

export async function updateTradeStatus(id, status) {
  const res = await fetch(`${API_URL}/api/trade/update`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-KEY': API_KEY
    },
    body: JSON.stringify({ id, status })
  });
  return await res.json();
}

export async function fetchApiDocs() {
  const res = await fetch(`${API_URL}/api/docs`);
  if (!res.ok) throw new Error('Failed to fetch API docs');
  return await res.json();
}

export async function fetchDashboard() {
  const res = await fetch(`${API_URL}/api/dashboard`);
  if (!res.ok) throw new Error('Failed to fetch dashboard');
  return await res.json();
}