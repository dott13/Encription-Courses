use std::error::Error;

/// PC-1 Permutation table for initial key permutation
const PC1: [u8; 56] = [
    57, 49, 41, 33, 25, 17,  9,  1,
    58, 50, 42, 34, 26, 18, 10,  2,
    59, 51, 43, 35, 27, 19, 11,  3,
    60, 52, 44, 36, 63, 55, 47, 39,
    31, 23, 15,  7, 62, 54, 46, 38,
    30, 22, 14,  6, 61, 53, 45, 37,
    29, 21, 13,  5, 28, 20, 12,  4
];

/// Key generation struct that can handle more flexible input
struct DesKeyGenerator {
    /// Raw input key
    raw_key: Vec<u8>,
    /// Processed 56-bit key
    k_plus: u64,
}

impl DesKeyGenerator {
    /// Create key from various input types
    fn new(input: &[u8]) -> Result<Self, Box<dyn Error>> {
        // Validate and process input
        let processed_key = Self::process_key(input)?;
        
        Ok(Self {
            raw_key: input.to_vec(),
            k_plus: processed_key,
        })
    }

    /// Flexible key processing method
    fn process_key(key_bytes: &[u8]) -> Result<u64, Box<dyn Error>> {
        // Different processing strategies based on input length
        match key_bytes.len() {
            // If exactly 8 bytes (standard DES key length)
            8 => Self::process_standard_key(key_bytes),
            
            // If less than 8 bytes, pad with zeros
            0..=7 => {
                let mut padded_key = vec![0u8; 8];
                padded_key[..key_bytes.len()].copy_from_slice(key_bytes);
                Self::process_standard_key(&padded_key)
            },
            
            // If more than 8 bytes, truncate
            _ => {
                let truncated_key = &key_bytes[..8];
                Self::process_standard_key(truncated_key)
            }
        }
    }

    /// Standard DES key processing with PC-1 permutation
    fn process_standard_key(key_bytes: &[u8]) -> Result<u64, Box<dyn Error>> {
        // Convert key to 64-bit integer
        let mut key_64bit: u64 = 0;
        for (i, &byte) in key_bytes.iter().enumerate() {
            key_64bit |= (byte as u64) << (56 - i * 8);
        }

        // Perform PC-1 permutation
        let mut k_plus: u64 = 0;
        for (i, &pos) in PC1.iter().enumerate() {
            let bit = (key_64bit >> (64 - pos)) & 1;
            k_plus |= bit << (55 - i);
        }

        Ok(k_plus)
    }

    /// Debugging method to print key details
    fn debug_print(&self) {
        println!("Raw Input (bytes): {:?}", self.raw_key);
        // Try to convert to a string, but handle non-UTF8 gracefully
        if let Ok(string_repr) = String::from_utf8(self.raw_key.clone()) {
            println!("Raw Input (as string): {}", string_repr);
        } else {
            println!("Raw Input (non-UTF8)");
        }
        println!("K+ Key (hex): 0x{:014X}", self.k_plus);
    }
}

fn main() {
    // Demonstrate flexible key generation
    let test_cases = vec![
        // Different types of inputs
        b"MORTYNOR".to_vec(),      // Standard 8-byte key
        b"SHORT".to_vec(),         // Less than 8 bytes (will be zero-padded)
        b"TOOLONGKEY123456".to_vec(), // More than 8 bytes (will be truncated)
        vec![],                    // Empty input (will be zero-padded)
        vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],  // Byte array
    ];

    for key in test_cases {
        match DesKeyGenerator::new(&key) {
            Ok(key_gen) => {
                println!("\n--- New Key Generation ---");
                key_gen.debug_print();
            }
            Err(e) => {
                eprintln!("Error generating key: {}", e);
            }
        }
    }
}