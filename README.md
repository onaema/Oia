# Proyek Tuyulxngepet

## Deskripsi
Proyek ini adalah aplikasi desktop berbasis Tauri dengan frontend menggunakan SvelteKit dan backend menggunakan Rust. Aplikasi ini dirancang untuk mendukung bot trading dengan fitur-fitur seperti pengelolaan strategi, analisis pasar, dan eksekusi transaksi.

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

## Fitur Utama
- **Backend**: Dibangun dengan Rust menggunakan framework Warp untuk API HTTP.
- **Frontend**: Dibangun dengan SvelteKit untuk antarmuka pengguna yang interaktif.
- **Integrasi**: Komunikasi antara frontend dan backend melalui API HTTP.
- **Pengujian**: Unit test dan integrasi untuk memastikan stabilitas.

## Cara Menjalankan
1. **Instalasi Dependensi**:
   ```bash
   npm install
   cd src-tauri && cargo build
   ```
2. **Jalankan Backend**:
   ```bash
   cd src-tauri
   cargo run
   ```
3. **Jalankan Frontend**:
   ```bash
   cd frontend
   npm run dev
   ```
4. **Akses Aplikasi**:
   - Frontend: `http://localhost:5000`
   - Backend API: `http://127.0.0.1:3030/api/hello`

## Pengujian
- **Frontend**:
  ```bash
  cd frontend
  npm run test
  ```
- **Backend**:
  ```bash
  cd src-tauri
  cargo test
  ```

## Kontribusi
Silakan buat pull request untuk kontribusi.
