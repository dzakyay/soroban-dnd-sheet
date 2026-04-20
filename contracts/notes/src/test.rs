#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test_character_creation_and_level_up() {
    // 1. Inisialisasi Environment Soroban
    let env = Env::default();
    let contract_id = env.register_contract(None, CharacterVault);
    let client = CharacterVaultClient::new(&env, &contract_id);

    // 2. Data Dummy Karakter (Misalnya untuk build Bard kamu)
    let char_id = 101;
    let name = String::from_str(&env, "Aethelgard");
    let char_class = String::from_str(&env, "Bard");
    let backstory = String::from_str(&env, "Seorang pemusik jalanan dari kota pelabuhan.");

    // 3. Eksekusi fungsi create_character
    client.create_character(&char_id, &name, &char_class, &backstory);

    // 4. Verifikasi apakah data tersimpan dengan benar
    let retrieved = client.get_character(&char_id);
    assert_eq!(retrieved.name, name);
    assert_eq!(retrieved.char_class, char_class);
    assert_eq!(retrieved.level, 1);

    // 5. Tes Fitur Level Up
    client.level_up(&char_id);
    let updated_char = client.get_character(&char_id);
    
    // Pastikan level bertambah menjadi 2
    assert_eq!(updated_char.level, 2);
}

#[test]
#[should_panic]
fn test_get_non_existent_character() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CharacterVault);
    let client = CharacterVaultClient::new(&env, &contract_id);

    // Mencoba mengambil ID yang belum pernah dibuat harusnya memicu panic
    client.get_character(&999);
}