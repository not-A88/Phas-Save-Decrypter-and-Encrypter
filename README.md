# Phas-Save-Decrypter-and-Encrypter

A simple Rust CLI tool to decrypt and encrypt Phasmophobia save files.

## Features
- **Decrypt**: Decrypts a Phasmophobia save file (`saveFile.txt`), removes the `playedMaps` field, and outputs a cleaned JSON file (`output/unencrypted_savefile.txt`).
- **Encrypt**: Encrypts a cleaned or modified save file (`output/unencrypted_savefile.txt`) back into the encrypted format (`output/savefile.txt`).

## Usage

1. **Build the project**
   ```sh
   cargo build --release
   ```

2. **Run the program**
   ```sh
   cargo run --release
   ```

3. **Choose an option**
   - `1`: Decrypt `saveFile.txt` (must be in the same directory as the executable)
     - Outputs cleaned JSON to `output/unencrypted_savefile.txt`
   - `2`: Encrypt `output/unencrypted_savefile.txt`
     - Outputs encrypted file to `output/savefile.txt`

## How it works
- **Decryption**
  - Reads `saveFile.txt`, extracts the IV, derives the key using PBKDF2-SHA1, and decrypts the data using AES-128-CBC.
  - Removes the `playedMaps` field from the JSON.
  - Writes the cleaned JSON to `output/unencrypted_savefile.txt`.

- **Encryption**
  - Reads `output/unencrypted_savefile.txt`.
  - Generates a random IV, derives the key using PBKDF2-SHA1, and encrypts the data using AES-128-CBC.
  - Prepends the IV to the ciphertext and writes the result to `output/savefile.txt`.

## Notes
- The tool will create the `output` directory automatically if it does not exist.
- If a required file is missing, the program will prompt and wait for you to press Enter before exiting.
- The password and encryption parameters are hardcoded for compatibility with Phasmophobia's save system.

## Example
```
Choose an option:
1. Decrypt saveFile.txt
2. Encrypt output/unencrypted_savefile.txt
Enter 1 or 2: 1
Decrypted and cleaned file written to output/unencrypted_savefile.txt
...
```

## License
MIT
