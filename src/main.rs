// We'll store encrypted passwords in a JSON file.
// Generate Encryption Key
// We use AES-GCM encryption for securing stored passwords:

use aes_gcm::{Aes256Gcm, Key, KeyInit};
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::aead::generic_array::GenericArray;
use rand::Rng; 
use rand::rngs::OsRng;
use rand_core::RngCore;
use rand::thread_rng;
use std::fs;
use std::collections::HashMap;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use serde_json;
use rpassword::prompt_password;



const KEY_FILE: &str = "keyfile";

// Retrieves or generates an encryption key used for password encryption.
fn get_key() -> Vec<u8> {
    let proj_dirs = ProjectDirs::from("com", "example", "password_manager").unwrap();
    let config_dir = proj_dirs.config_dir(); // Get the config directory
    let key_path = config_dir.join(KEY_FILE);

    // Ensure the directory exists before writing the file
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }

    if key_path.exists() {
        return fs::read(&key_path).expect("Failed to read keyfile");
    }

    let key: [u8; 32] = thread_rng().gen();
    fs::write(&key_path, &key).expect("Failed to write keyfile");
    key.to_vec()
}

// Encrypts a given password using AES-256-GCM.
fn encrypt_password(password: &str, key: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let mut nonce_bytes = [0u8; 12]; 
    OsRng.fill_bytes(&mut nonce_bytes); 
    let nonce = GenericArray::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, password.as_bytes()).expect("Encryption failed");

    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend(ciphertext);
    encrypted_data
}

// Decrypts an encrypted password.
fn decrypt_password(encrypted_data: &[u8], key: &[u8]) -> String {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = GenericArray::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("Decryption failed");
    String::from_utf8(plaintext).expect("Invalid UTF-8")
}

// Store encrypted credentials in a JSON file:
#[derive(Serialize, Deserialize)]
struct PasswordEntry {
    service: String,
    encrypted_password: Vec<u8>,
}

// Encrypts and saves a password for a given service.
fn save_password(service: &str, password: &str, key: &[u8]) {
    let encrypted_password = encrypt_password(password, key);
    let entry = PasswordEntry {
        service: service.to_string(),
        encrypted_password,
    };

    let proj_dirs = ProjectDirs::from("com", "example", "password_manager").unwrap();
    let data_dir = proj_dirs.data_dir();
    let data_path = data_dir.join("passwords.json");

    // Ensure the directory exists before writing the file
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("Failed to create data directory");
    }

    let mut passwords: HashMap<String, PasswordEntry> = if data_path.exists() {
        let data = fs::read_to_string(&data_path).expect("Failed to read passwords file");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    };

    passwords.insert(service.to_string(), entry);
    fs::write(&data_path, serde_json::to_string(&passwords).unwrap()).expect("Failed to write passwords file");
}

// Retrieves and decrypts a stored password.
fn retrieve_password(service: &str, key: &[u8]) -> Option<String> {
    let proj_dirs = ProjectDirs::from("com", "example", "password_manager").unwrap();
    let data_path = proj_dirs.data_dir().join("passwords.json");

    if !data_path.exists() {
        return None;
    }

    let data = fs::read_to_string(data_path).expect("Failed to read passwords file");
    let passwords: HashMap<String, PasswordEntry> = serde_json::from_str(&data).unwrap_or_default();

    passwords.get(service).map(|entry| decrypt_password(&entry.encrypted_password, key))
}

// Provides a simple Command-Line Interface (CLI) for saving and retrieving passwords.
fn main() {
    let key = get_key();

    println!("Choose an option: 1) Save Password 2) Retrieve Password");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            println!("Enter service name:");
            let mut service = String::new();
            std::io::stdin().read_line(&mut service).unwrap();

            let password = prompt_password("Enter password: ").unwrap();
            save_password(service.trim(), &password, &key);
            println!("Password saved successfully!");
        }
        "2" => {
            println!("Enter service name:");
            let mut service = String::new();
            std::io::stdin().read_line(&mut service).unwrap();

            if let Some(password) = retrieve_password(service.trim(), &key) {
                println!("Retrieved password: {}", password);
            } else {
                println!("Service not found!");
            }
        }
        _ => println!("Invalid option!"),
    }
}

// Compile and run the program: 
// cargo run
// Usage:
// 1. select 1 to save password
// 2. select 2 to retrieve a stored password