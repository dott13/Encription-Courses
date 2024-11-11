use std::collections::HashSet;
use std::io::{self, Write};

fn validate_text(text: &str) -> bool {
    text.chars().all(|c| c.is_alphabetic() || "ăâîșț".contains(c))
}

fn validate_key(key: &str) -> bool {
    key.len() >= 7 && validate_text(key)
}

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

fn create_matrix(key: &str) -> Vec<Vec<char>> {
    // Create a flexible-sized matrix to accommodate all characters
    let mut matrix = Vec::new();
    let mut current_row = Vec::new();
    
    // Process the key first
    let key_processed = remove_duplicates(&key.to_uppercase().replace('J', "I"));
    let mut all_chars: Vec<char> = key_processed.chars().collect();
    
    // Add remaining alphabet and Romanian characters
    let alphabet = "ABCDEFGHIKLMNOPQRSTUVWXYZĂÂÎȘȚ";
    for c in alphabet.chars() {
        if !all_chars.contains(&c) {
            all_chars.push(c);
        }
    }

    // Create the matrix with 5 columns
    for (_idx, &c) in all_chars.iter().enumerate() {
        current_row.push(c);
        if current_row.len() == 5 {
            matrix.push(current_row);
            current_row = Vec::new();
        }
    }
    
    // Push the last row if it exists
    if !current_row.is_empty() {
        while current_row.len() < 5 {
            current_row.push('X');  // Fill with X if needed
        }
        matrix.push(current_row);
    }

    matrix
}

fn find_position(matrix: &[Vec<char>], c: char) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &matrix_char) in row.iter().enumerate() {
            if matrix_char == c {
                return Some((i, j));
            }
        }
    }
    None
}

fn encrypt_playfair(matrix: &[Vec<char>], text: &str) -> String {
    let text = text.to_uppercase().replace('J', "I");
    let mut text_chars: Vec<char> = text.chars().collect();
    
    // Add padding if necessary
    if text_chars.len() % 2 != 0 {
        text_chars.push('X');
    }

    let mut result = String::new();
    let rows = matrix.len();

    for chunk in text_chars.chunks(2) {
        let (c1, c2) = (chunk[0], chunk[chunk.len() - 1]);
        
        if let (Some((r1, c1_pos)), Some((r2, c2_pos))) = (find_position(matrix, c1), find_position(matrix, c2)) {
            if r1 == r2 {
                // Same row
                result.push(matrix[r1][(c1_pos + 1) % 5]);
                result.push(matrix[r2][(c2_pos + 1) % 5]);
            } else if c1_pos == c2_pos {
                // Same column
                result.push(matrix[(r1 + 1) % rows][c1_pos]);
                result.push(matrix[(r2 + 1) % rows][c2_pos]);
            } else {
                // Rectangle
                result.push(matrix[r1][c2_pos]);
                result.push(matrix[r2][c1_pos]);
            }
        } else {
            // If character not found, append it unchanged
            result.push(c1);
            if chunk.len() > 1 {
                result.push(c2);
            }
        }
    }
    result
}

fn decrypt_playfair(matrix: &[Vec<char>], text: &str) -> String {
    let text = text.to_uppercase();
    let mut result = String::new();
    let rows = matrix.len();

    for chunk in text.chars().collect::<Vec<char>>().chunks(2) {
        let (c1, c2) = (chunk[0], chunk[chunk.len() - 1]);
        
        if let (Some((r1, c1_pos)), Some((r2, c2_pos))) = (find_position(matrix, c1), find_position(matrix, c2)) {
            if r1 == r2 {
                // Same row
                result.push(matrix[r1][(c1_pos + 4) % 5]);
                result.push(matrix[r2][(c2_pos + 4) % 5]);
            } else if c1_pos == c2_pos {
                // Same column
                result.push(matrix[(r1 + rows - 1) % rows][c1_pos]);
                result.push(matrix[(r2 + rows - 1) % rows][c2_pos]);
            } else {
                // Rectangle
                result.push(matrix[r1][c2_pos]);
                result.push(matrix[r2][c1_pos]);
            }
        } else {
            // If character not found, append it unchanged
            result.push(c1);
            if chunk.len() > 1 {
                result.push(c2);
            }
        }
    }
    result
}

fn get_valid_operation() -> io::Result<u32> {
    loop {
        print!("Choose an operation (1: Encrypt, 2: Decrypt): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u32>() {
            Ok(num) if num == 1 || num == 2 => return Ok(num),
            _ => {
                println!("Please enter either 1 for encryption or 2 for decryption.");
                continue;
            }
        }
    }
}

fn get_valid_key() -> io::Result<String> {
    loop {
        print!("Enter the key (at least 7 characters): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let key = input.trim().to_string();
        
        if validate_key(&key) {
            return Ok(key);
        } else {
            println!("The key must be at least 7 characters long and contain only letters (including Romanian ones)!");
        }
    }
}

fn get_valid_message() -> io::Result<String> {
    loop {
        print!("Enter the message for encryption/decryption: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let message = input.trim().to_string();
        
        if validate_text(&message) {
            return Ok(message);
        } else {
            println!("The message must contain only letters (including Romanian ones)!");
        }
    }
}

fn main() -> io::Result<()> {
    println!("=== Playfair Cipher (with Romanian character support) ===\n");
    
    let key = get_valid_key()?;
    let matrix = create_matrix(&key);
    
    // Debug: Print the matrix
    println!("\nPlayfair Matrix:");
    for row in &matrix {
        println!("{:?}", row);
    }
    println!();
    
    let operation = get_valid_operation()?;
    let message = get_valid_message()?;
    
    match operation {
        1 => {
            let ciphertext = encrypt_playfair(&matrix, &message);
            println!("\nEncrypted text: {}", ciphertext);
        },
        2 => {
            let decrypted_message = decrypt_playfair(&matrix, &message);
            println!("\nDecrypted message: {}", decrypted_message);
        },
        _ => unreachable!()
    }
    
    Ok(())
}