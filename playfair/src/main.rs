use std::collections::HashSet;
use std::io::{self};

// Function to validate the input (only letters and between 'A' and 'Z')
fn validate_text(text: &str) -> bool {
    text.chars().all(|c| c.is_alphabetic())
}

// Function to validate the key (at least 7 characters)
fn validate_key(key: &str) -> bool {
    key.len() >= 7 && validate_text(key)
}

// Function to remove duplicate characters from the key
fn remove_duplicates(key: &str) -> String {
    let mut result = String::new();
    let mut seen = HashSet::new();

    for c in key.chars() {
        if !seen.contains(&c) {
            result.push(c);
            seen.insert(c);
        }
    }
    result
}

// Function to create the Playfair matrix from the given key
fn create_matrix(key: &str) -> [[char; 5]; 5] {
    let alphabet = "ABCDEFGHIKLMNOPQRSTUVWXYZ";  // 'J' is excluded
    let mut key_without_duplicates = remove_duplicates(key.to_uppercase().replace('J', "I").as_str());

    for c in alphabet.chars() {
        if !key_without_duplicates.contains(c) {
            key_without_duplicates.push(c);
        }
    }

    // Initialize a 5x5 matrix
    let mut playfair_matrix = [[' '; 5]; 5];
    let mut index = 0;

    for i in 0..5 {
        for j in 0..5 {
            playfair_matrix[i][j] = key_without_duplicates.chars().nth(index).unwrap();
            index += 1;
        }
    }

    playfair_matrix
}

// Function to encrypt using Playfair cipher
fn encrypt_playfair(matrix: &[[char; 5]; 5], text: &str) -> String {
    let mut text = text.to_uppercase().replace('J', "I"); // Replace J with I
    if text.len() % 2 != 0 {
        text.push('X'); // Add 'X' if the text has an odd number of characters
    }

    let mut ciphertext = String::new();

    // Pair the characters and process
    for i in 0..text.len() / 2 {
        let a = text.chars().nth(2 * i).unwrap();
        let b = text.chars().nth(2 * i + 1).unwrap();
        let (x1, y1) = find_position(matrix, a);
        let (x2, y2) = find_position(matrix, b);

        if x1 == x2 {
            // On the same row
            ciphertext.push(matrix[x1][(y1 + 1) % 5]);
            ciphertext.push(matrix[x2][(y2 + 1) % 5]);
        } else if y1 == y2 {
            // On the same column
            ciphertext.push(matrix[(x1 + 1) % 5][y1]);
            ciphertext.push(matrix[(x2 + 1) % 5][y2]);
        } else {
            // Form a rectangle
            ciphertext.push(matrix[x1][y2]);
            ciphertext.push(matrix[x2][y1]);
        }
    }
    ciphertext
}

// Function to decrypt using Playfair cipher
fn decrypt_playfair(matrix: &[[char; 5]; 5], text: &str) -> String {
    let text = text.to_uppercase().replace('J', "I");

    let mut decrypted_message = String::new();

    // Pair the characters and process
    for i in 0..text.len() / 2 {
        let a = text.chars().nth(2 * i).unwrap();
        let b = text.chars().nth(2 * i + 1).unwrap();
        let (x1, y1) = find_position(matrix, a);
        let (x2, y2) = find_position(matrix, b);

        if x1 == x2 {
            // On the same row
            decrypted_message.push(matrix[x1][(y1 + 4) % 5]);
            decrypted_message.push(matrix[x2][(y2 + 4) % 5]);
        } else if y1 == y2 {
            // On the same column
            decrypted_message.push(matrix[(x1 + 4) % 5][y1]);
            decrypted_message.push(matrix[(x2 + 4) % 5][y2]);
        } else {
            // Form a rectangle
            decrypted_message.push(matrix[x1][y2]);
            decrypted_message.push(matrix[x2][y1]);
        }
    }
    decrypted_message
}

// Function to find the position of a character in the matrix
fn find_position(matrix: &[[char; 5]; 5], c: char) -> (usize, usize) {
    for i in 0..5 {
        for j in 0..5 {
            if matrix[i][j] == c {
                return (i, j);
            }
        }
    }
    (0, 0) // Will return 0, 0 as a fallback, but this should never happen
}

fn main() {
    let mut input = String::new();
    println!("Enter the key (at least 7 characters): ");
    io::stdin().read_line(&mut input).unwrap();
    let key = input.trim();
    
    if !validate_key(key) {
        println!("The key must be at least 7 characters long and contain only letters!");
        return;
    }

    let matrix = create_matrix(key);
    
    println!("Choose an operation (1: Encrypt, 2: Decrypt): ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let operation: u32 = input.trim().parse().unwrap();

    println!("Enter the message for encryption/decryption: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let message = input.trim();

    if !validate_text(message) {
        println!("The message must contain only letters!");
        return;
    }

    if operation == 1 {
        let ciphertext = encrypt_playfair(&matrix, message);
        println!("Ciphertext: {}", ciphertext);
    } else if operation == 2 {
        let decrypted_message = decrypt_playfair(&matrix, message);
        println!("Decrypted message: {}", decrypted_message);
    } else {
        println!("Invalid option!");
    }
}
