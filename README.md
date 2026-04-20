# Stellar Cashier DApp

**Stellar Cashier DApp** - Sistem Kasir (Point of Sale) Terdesentralisasi Berbasis Blockchain

## Deskripsi Proyek

Stellar Cashier DApp adalah solusi *smart contract* terdesentralisasi yang dibangun di atas blockchain Stellar menggunakan Soroban SDK. Aplikasi ini menyediakan platform yang aman dan tidak dapat diubah (*immutable*) untuk mencatat riwayat transaksi penjualan langsung di blockchain. *Smart contract* ini memastikan bahwa data penjualan bisnis kamu disimpan secara transparan dan hanya dapat dikelola melalui fungsi yang telah ditentukan, menghilangkan ketergantungan pada *database* terpusat yang rentan terhadap manipulasi atau kehilangan data.

Sistem ini memungkinkan kasir untuk mencatat transaksi baru, melihat riwayat penjualan, dan membatalkan transaksi, dengan memanfaatkan efisiensi dan keamanan jaringan Stellar. Setiap transaksi diidentifikasi secara unik dan disimpan di dalam *storage instance contract*, memastikan persistensi dan keandalan data finansial.

## Visi Proyek

Visi kami adalah merevolusi sistem pencatatan keuangan dan kasir di era digital dengan cara:

- **Mendesentralisasi Data Penjualan**: Memindahkan catatan transaksi dari server terpusat ke blockchain global yang terdistribusi.
- **Mencegah Manipulasi Data (Fraud)**: Memberikan jaminan kepada pemilik bisnis bahwa data penjualan tidak dapat diubah secara diam-diam oleh pihak yang tidak bertanggung jawab.
- **Menjamin Imutabilitas**: Menyediakan catatan permanen dari setiap transaksi masuk dan keluar.
- **Membangun Sistem Tanpa Kepercayaan Dasar (*Trustless*)**: Menciptakan platform di mana integritas data finansial dijamin oleh kode, bukan oleh pelaporan manual.

Kami membayangkan masa depan di mana sistem POS (*Point of Sale*) dapat beroperasi secara otonom, transparan, dan dapat diaudit secara *real-time* oleh pemilik bisnis di mana saja.

## Fitur Utama

### 1. **Pencatatan Transaksi Otomatis**
- Catat penjualan hanya dengan satu pemanggilan fungsi.
- Masukkan nama barang, jumlah, dan harga satuan; *smart contract* akan menghitung total harga secara otomatis.
- Pembuatan ID unik (*auto-generated*) untuk setiap resi/transaksi.

### 2. **Pengambilan Data yang Efisien**
- Ambil seluruh riwayat transaksi yang tersimpan dalam satu pemanggilan.
- Representasi data yang terstruktur untuk memudahkan integrasi dengan *frontend* (dashboard admin/kasir).
- Sinkronisasi *real-time* dengan *state* di blockchain.

### 3. **Pembatalan yang Terlacak (Aman)**
- Hapus atau batalkan transaksi spesifik menggunakan ID uniknya (misal: untuk kasus *refund* atau salah input).
- Pembaruan daftar transaksi seketika setelah pembatalan dilakukan.

### 4. **Transparansi dan Auditabilitas**
- Lihat semua aktivitas kasir di *ledger* blockchain.
- Verifikasi berbasis blockchain untuk semua tindakan pencatatan keuangan.
- Rekam jejak *immutable* yang sangat cocok untuk keperluan pembukuan dan audit.

### 5. **Integrasi Jaringan Stellar**
- Memanfaatkan kecepatan tinggi dan biaya transaksi yang sangat rendah dari jaringan Stellar.
- Dibangun menggunakan Soroban Smart Contract SDK modern.
- Arsitektur yang *scalable* untuk volume transaksi toko yang terus bertambah.

## Detail Kontrak

- **Alamat Kontrak (Contoh):** `CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M`

## Rencana Pengembangan (Future Scope)

### Jangka Pendek (Short-Term Enhancements)
1. **Manajemen Inventaris**: Menambahkan fitur untuk melacak dan mengurangi stok barang secara otomatis saat transaksi terjadi.
2. **Katalog Produk**: Menyimpan daftar harga dan nama produk di dalam *state* agar kasir hanya perlu memasukkan ID produk.
3. **Diskon & Pajak**: Dukungan kalkulasi persentase diskon dan PPN sebelum total harga final disimpan.

### Jangka Menengah (Medium-Term Development)
4. **Pembayaran Langsung (On-Chain Settlement)**: Mengintegrasikan token/Stablecoin (seperti USDC di jaringan Stellar) agar pelanggan bisa langsung membayar transaksi melalui *crypto wallet*.
5. **Multi-Kasir (Role-Based Access)**: Mengizinkan beberapa *address* (kasir) untuk menginput data, namun hanya *address* pemilik toko (Admin) yang bisa menghapus data.
6. **Resi Digital**: Jembatan *off-chain* untuk menghasilkan resi digital yang bisa dikirim ke pelanggan.

### Jangka Panjang (Long-Term Vision)
7. **Program Loyalitas Web3**: Mengotomatisasi pemberian token *rewards* atau NFT diskon kepada pelanggan tetap setiap kali mereka bertransaksi.
8. **Prediksi Penjualan berbasis AI**: Menggunakan data *on-chain* untuk dianalisis oleh AI *off-chain* guna memprediksi tren barang terlaris.
9. **UI Terdesentralisasi**: Melakukan *hosting frontend* kasir di IPFS.

### Fitur *Enterprise* (Skala Besar)
10. **Sinkronisasi Multi-Cabang**: Mengadaptasi sistem untuk jaringan ritel besar dengan identifikasi transaksi antar cabang.
11. **Pelaporan Pajak Otomatis**: *Trigger* otomatis yang merangkum omset bulanan untuk mempermudah pelaporan pajak.
12. **Integrasi Rantai Pasok (Supply Chain)**: Menghubungkan data penjualan dengan kontrak pintar *supplier* untuk *restock* barang secara otomatis.

---

## Persyaratan Teknis

- Soroban SDK
- Bahasa Pemrograman Rust
- Jaringan Blockchain Stellar (Futurenet / Testnet / Mainnet)

## Memulai Cepat (Getting Started)

Lakukan *deploy smart contract* ini ke jaringan Soroban milik Stellar dan berinteraksilah menggunakan tiga fungsi utama berikut:

- `add_transaction(item_name, quantity, price_per_item)` - Mencatat transaksi baru. Total harga akan dihitung secara otomatis di dalam kontrak.
- `get_transactions()` - Mengambil daftar semua transaksi penjualan yang telah dicatat.
- `delete_transaction(id)` - Menghapus atau membatalkan transaksi tertentu berdasarkan ID transaksinya.

---

**Stellar Cashier DApp** - Mengamankan dan Memodernisasi Pembukuan Bisnismu di atas Blockchain

ID SmartContract : CAKO5H2EN2OQD4QTRW43QYPBTW2ZUKVQ7R5SC5D6UIJWSMV6SNS2UN4W