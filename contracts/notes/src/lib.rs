#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan detail transaksi kasir
#[contracttype]
#[derive(Clone, Debug)]
pub struct Transaction {
    id: u64,
    item_name: String,
    quantity: u32,
    total_price: u32,
}

// Storage key untuk data transaksi
const TX_DATA: Symbol = symbol_short!("TX_DATA");

#[contract]
pub struct CashierContract;

#[contractimpl]
impl CashierContract {
    // Fungsi untuk melihat semua riwayat transaksi
    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        // 1. Ambil data transaksi dari storage
        return env.storage().instance().get(&TX_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk mencatat transaksi baru (kasir memasukkan barang)
    pub fn add_transaction(env: Env, item_name: String, quantity: u32, price_per_item: u32) -> String {
        // 1. Ambil data transaksi dari storage
        let mut transactions: Vec<Transaction> = env.storage().instance().get(&TX_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Hitung total harga (jumlah barang x harga satuan)
        let total_price = quantity * price_per_item;

        // 3. Buat object transaksi baru
        let tx = Transaction {
            id: env.prng().gen::<u64>(),
            item_name: item_name,
            quantity: quantity,
            total_price: total_price,
        };
        
        // 4. Tambahkan transaksi baru ke daftar yang sudah ada
        transactions.push_back(tx);
        
        // 5. Simpan pembaruan ke storage
        env.storage().instance().set(&TX_DATA, &transactions);
        
        return String::from_str(&env, "Transaksi berhasil dicatat");
    }

    // Fungsi untuk membatalkan/menghapus transaksi berdasarkan ID
    pub fn delete_transaction(env: Env, id: u64) -> String {
        // 1. Ambil data transaksi dari storage 
        let mut transactions: Vec<Transaction> = env.storage().instance().get(&TX_DATA).unwrap_or(Vec::new(&env));

        // 2. Cari index transaksi yang akan dihapus menggunakan perulangan
        for i in 0..transactions.len() {
            if transactions.get(i).unwrap().id == id {
                transactions.remove(i);

                // 3. Simpan state terbaru ke storage setelah dihapus
                env.storage().instance().set(&TX_DATA, &transactions);
                return String::from_str(&env, "Transaksi berhasil dibatalkan");
            }
        }

        return String::from_str(&env, "Data transaksi tidak ditemukan");
    }
}

mod test;