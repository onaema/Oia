# Solana Memecoin Bot

## Deskripsi
Proyek ini adalah aplikasi desktop berbasis Tauri dengan frontend menggunakan SvelteKit dan backend menggunakan Rust. Aplikasi ini dirancang untuk mendukung bot trading dengan fitur-fitur seperti pengelolaan strategi, analisis pasar, dan eksekusi transaksi.

## Fitur Utama
- Trading otomatis (simulasi, siap integrasi Solana nyata)
- Dashboard statistik & riwayat trade
- Notifikasi & alert
- API backend lengkap (autentikasi X-API-KEY)
- Frontend Svelte terintegrasi

## Endpoint API

| Method | Path                   | Deskripsi                                    |
|--------|------------------------|----------------------------------------------|
| GET    | /api/hello             | Cek koneksi backend                          |
| GET    | /api/status            | Status backend & waktu server                |
| GET    | /api/dashboard         | Statistik trading                            |
| GET    | /api/trades            | Riwayat trade (X-API-KEY)                    |
| POST   | /api/trade             | Eksekusi trade (X-API-KEY, body: symbol, amount, side) |
| POST   | /api/trade/cancel      | Cancel trade by id (X-API-KEY, body: id)     |
| POST   | /api/trade/update      | Update status trade by id (X-API-KEY, body: id, status) |
| GET    | /api/wallet/balance    | Cek saldo wallet (X-API-KEY)                 |
| GET    | /api/notify/{msg}      | Kirim notifikasi (log)                       |
| GET    | /api/docs              | Dokumentasi endpoint API                     |

**Autentikasi:**
Gunakan header `X-API-KEY: secret` (atau sesuai .env/API_KEY) untuk endpoint trade, wallet, dan trades.

## Contoh Request API

### Trade (POST)
```bash
curl -X POST http://127.0.0.1:3030/api/trade \
  -H 'Content-Type: application/json' \
  -H 'X-API-KEY: secret' \
  -d '{"symbol":"SOL/USDC","amount":1,"side":"buy"}'
```

### Cek Saldo Wallet
```bash
curl -H 'X-API-KEY: secret' http://127.0.0.1:3030/api/wallet/balance
```

### Riwayat Trade
```bash
curl -H 'X-API-KEY: secret' http://127.0.0.1:3030/api/trades
```

### Cancel Trade
```bash
curl -X POST http://127.0.0.1:3030/api/trade/cancel \
  -H 'Content-Type: application/json' \
  -H 'X-API-KEY: secret' \
  -d '{"id":123456789}'
```

### Update Status Trade
```bash
curl -X POST http://127.0.0.1:3030/api/trade/update \
  -H 'Content-Type: application/json' \
  -H 'X-API-KEY: secret' \
  -d '{"id":123456789,"status":"Selesai"}'
```

## Struktur Proyek
```
📦 Proyek
┣ 📂 src-tauri/ (Backend Native)
┃ ┣ 🔷 main.rs (Entry point backend)
┃ ┣ 🔷 bot_bridge.rs (Pengelolaan bot)
┃ ┗ 🔷 Cargo.toml (Konfigurasi Rust)
┣ 📂 frontend/ (Frontend SvelteKit)
┃ ┣ 📂 routes/ (Halaman aplikasi)
┃ ┣ 📂 lib/ (Komponen UI)
┃ ┣ 📂 utils/ (Fungsi utilitas)
┃ ┗ 🔷 rollup.config.js (Konfigurasi build)
```

## Cara Menjalankan

### Backend
```bash
cargo run
```

### Frontend
```bash
cd frontend
npm install
npm run dev
```

### Build Production (Docker)
```bash
docker-compose up --build
```

### Testing
- Frontend: `cd frontend && npm run test`
- Backend: `cargo test`

## Deployment Instructions

### Prerequisites
- Docker and Docker Compose installed on your system.

### Steps to Deploy
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-folder>
   ```

2. Build and run the services using Docker Compose:
   ```bash
   docker-compose up --build
   ```

3. Access the services:
   - Backend: [http://localhost:3030](http://localhost:3030)
   - Frontend: [http://localhost](http://localhost)

### Notes
- Ensure that the `config.json` file is properly configured for the backend.
- Modify the `Dockerfile` or `docker-compose.yml` if you need to customize the deployment.

## CI/CD & Deployment

Pipeline otomatis sudah tersedia di `.github/workflows/ci-cd.yml`:
- Build & test backend (Rust)
- Build & test frontend (Svelte)
- Build Docker image backend & frontend

### Deploy ke Docker Hub (Opsional)
Untuk push image ke Docker Hub, tambahkan langkah berikut pada job backend/frontend di workflow:
```yaml
- name: Login to Docker Hub
  uses: docker/login-action@v3
  with:
    username: ${{ secrets.DOCKERHUB_USERNAME }}
    password: ${{ secrets.DOCKERHUB_TOKEN }}
- name: Push Docker Image
  run: docker push <username>/solana-memecoin-bot-backend
```

### Deploy ke Server/VPS
1. Pastikan server sudah terinstall Docker & docker-compose.
2. Clone repo dan jalankan:
```bash
docker-compose up --build -d
```
3. Akses backend di http://<server-ip>:3030 dan frontend di http://<server-ip>

### Otomatisasi Lanjutan
- Tambahkan secrets di GitHub untuk Docker Hub atau server.
- Tambahkan notifikasi (misal: ke Telegram/Slack) pada workflow jika build gagal/berhasil.
- Integrasi dengan layanan cloud (misal: AWS ECS, GCP Cloud Run, Azure Container Apps) jika ingin deployment cloud native.

## Struktur Folder
- src/: Backend Rust
- frontend/: Frontend Svelte
- trades.json: Data riwayat trade

## Catatan
- Untuk trading nyata di blockchain, integrasikan private key dan logic transaksi di integration/solana.rs.
- Semua endpoint utama sudah siap untuk dihubungkan ke frontend atau tools lain.

## Kontribusi
Silakan buat pull request untuk kontribusi.
