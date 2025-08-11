use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use pbkdf2::pbkdf2_hmac_array;
use rand::rngs::OsRng;
use rand::RngCore;
use regex::Regex;
use serde_json::Value;
use sha1::Sha1;
use std::fs;
use std::io::{self, Write};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const PASSWORD: &[u8] = b"t36gref9u84y7f43g";

fn encrypt_file() {
    let input_path = "output/unencrypted_saveFIle.txt";
    let output_path = "output/saveFIle.txt";

    let plaintext = match fs::read(input_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not read {}. Press Enter to exit...", input_path);
            let _ = io::stdin().read_line(&mut String::new());
            return;
        }
    };


    // Try to get IV from saveFile.txt if it exists
    let mut iv = [0u8; 16];
    let savefile_path = "saveFile.txt";
    let iv_from_savefile = fs::read(savefile_path)
        .ok()
        .and_then(|data| {
            if data.len() >= 16 {
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&data[..16]);
                Some(arr)
            } else {
                None
            }
        });
    if let Some(existing_iv) = iv_from_savefile {
        iv = existing_iv;
        println!("Using IV from saveFile.txt");
    } else {
        OsRng.fill_bytes(&mut iv);
        println!("Generated new random IV");
    }

    let key = pbkdf2_hmac_array::<Sha1, 16>(PASSWORD, &iv, 100);

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(&plaintext);

    let mut output = Vec::from(iv);
    output.extend_from_slice(&ciphertext);

    fs::write(output_path, &output).unwrap();

    println!("Encrypted file written to {}", output_path);
    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn decrypt_file(data: &[u8]) -> Option<Value> {
    if data.is_empty() {
        return None;
    }

    let iv = &data[..16];

    let key = pbkdf2_hmac_array::<Sha1, 16>(PASSWORD, iv, 100);

    let cipher = Aes128Cbc::new_from_slices(&key, iv).unwrap();
    let decrypted = cipher.decrypt_vec(&data[16..]).unwrap();

    let decoded_text = String::from_utf8_lossy(&decrypted);

    let played_maps_regex = Regex::new(
        r#",?\s*"playedMaps"\s*:\s*\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\}\s*"#
    ).unwrap();
    let fixed_text = played_maps_regex.replace_all(&decoded_text, "");

    let _ = fs::create_dir_all("output");

    fs::write("output/unencrypted_saveFIle.txt", fixed_text.as_bytes()).unwrap();

    println!("{}", fixed_text);

    let parsed: Value = serde_json::from_str(&fixed_text).unwrap();
    Some(parsed)
}

fn main() {
    println!("Choose an option:");
    println!("1. Decrypt saveFile.txt");
    println!("2. Encrypt output/unencrypted_saveFIle.txt");
    print!("Enter 1 or 2: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            let save_data = match fs::read("saveFile.txt") {
                Ok(data) => data,
                Err(_) => {
                    println!("Could not find saveFile.txt. Press Enter to exit...");
                    let _ = io::stdin().read_line(&mut String::new());
                    return;
                }
            };

            let parsed = decrypt_file(&save_data);
            println!("{:#?}", parsed);
        }
        "2" => {
            encrypt_file();
        }
        _ => {
            println!("Invalid option. Press Enter to exit...");
            let _ = io::stdin().read_line(&mut String::new());
        }
    }
}
