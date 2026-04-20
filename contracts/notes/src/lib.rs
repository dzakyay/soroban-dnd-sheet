#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String};

// 1. Definisi Struktur Lembar Karakter
#[contracttype]
#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub char_class: String,
    pub level: u32,
    pub backstory: String,
}

#[contract]
pub struct CharacterVault;

#[contractimpl]
impl CharacterVault {
    // 2. Fungsi membuat karakter baru
    pub fn create_character(env: Env, id: u32, name: String, char_class: String, backstory: String) {
        let character = Character {
            name,
            char_class,
            level: 1, // Semua karakter baru dimulai dari level 1
            backstory,
        };

        // Menyimpan data karakter ke blockchain menggunakan ID sebagai kunci
        env.storage().persistent().set(&id, &character);
    }

    // 3. Fungsi mengambil data karakter
    pub fn get_character(env: Env, id: u32) -> Character {
        // Mengambil data berdasarkan ID, akan panic jika ID tidak ditemukan
        env.storage().persistent().get(&id).unwrap()
    }
    
    // 4. Fungsi khusus untuk Level Up
    pub fn level_up(env: Env, id: u32) {
        // Tarik data lama
        let mut character: Character = env.storage().persistent().get(&id).unwrap();
        
        // Naikkan stat
        character.level += 1;
        
        // Simpan kembali data yang sudah diperbarui
        env.storage().persistent().set(&id, &character);
    }
}