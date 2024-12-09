use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use std::env;
use std::io::{self, Write};

/// PKI Configuration Structure
struct PKIConfig {
    ca_key_bits: u32,
    user_key_bits: u32,
    ca_validity_days: u32,
    user_validity_days: u32,
    ca_dir: String,
    users_dir: String,
}

impl PKIConfig {
    fn new() -> Self {
        PKIConfig {
            ca_key_bits: 4096,
            user_key_bits: 2048,
            ca_validity_days: 3650,
            user_validity_days: 365,
            ca_dir: String::from("./pki/ca"),
            users_dir: String::from("./pki/users"),
        }
    }

    /// Initialize PKI directory structure
    fn init_pki_structure(&self) -> io::Result<()> {
        fs::create_dir_all(&self.ca_dir)?;
        fs::create_dir_all(&self.users_dir)?;
        Ok(())
    }

    /// Generate CA Private Key
    fn generate_ca_key(&self) -> io::Result<()> {
        let ca_key_path = format!("{}/ca_private_key.pem", self.ca_dir);
        
        let output = Command::new("openssl")
            .args(&[
                "genrsa", 
                "-out", &ca_key_path, 
                &self.ca_key_bits.to_string()
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                "Failed to generate CA private key"
            ));
        }

        Ok(())
    }

    /// Create Self-Signed CA Certificate
    fn create_ca_certificate(&self) -> io::Result<()> {
        let ca_key_path = format!("{}/ca_private_key.pem", self.ca_dir);
        let ca_cert_path = format!("{}/ca_certificate.pem", self.ca_dir);
        
        let output = Command::new("openssl")
            .args(&[
                "req", "-x509", "-new", "-nodes",
                "-key", &ca_key_path,
                "-sha256",
                "-days", &self.ca_validity_days.to_string(),
                "-out", &ca_cert_path,
                "-subj", "/CN=DotUnity CA/O=DotCompany/OU=IT Department"
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                "Failed to create CA self-signed certificate"
            ));
        }

        Ok(())
    }

    /// Generate User Private Key
    fn generate_user_key(&self, username: &str) -> io::Result<()> {
        let user_key_path = format!("{}/{}_private_key.pem", self.users_dir, username);
        
        let output = Command::new("openssl")
            .args(&[
                "genrsa", 
                "-out", &user_key_path, 
                &self.user_key_bits.to_string()
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("Failed to generate private key for user {}", username)
            ));
        }

        Ok(())
    }

    /// Generate Certificate Signing Request (CSR)
    fn generate_csr(&self, username: &str) -> io::Result<()> {
        let user_key_path = format!("{}/{}_private_key.pem", self.users_dir, username);
        let user_csr_path = format!("{}/{}_csr.pem", self.users_dir, username);
        
        let output = Command::new("openssl")
            .args(&[
                "req", "-new", 
                "-key", &user_key_path,
                "-out", &user_csr_path,
                "-subj", &format!("/CN={}/O=MyOrganization", username)
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("Failed to generate CSR for user {}", username)
            ));
        }

        Ok(())
    }

    /// Sign User Certificate
    fn sign_user_certificate(&self, username: &str) -> io::Result<()> {
        let ca_key_path = format!("{}/ca_private_key.pem", self.ca_dir);
        let ca_cert_path = format!("{}/ca_certificate.pem", self.ca_dir);
        let user_csr_path = format!("{}/{}_csr.pem", self.users_dir, username);
        let user_cert_path = format!("{}/{}_certificate.pem", self.users_dir, username);
        
        let output = Command::new("openssl")
            .args(&[
                "x509", "-req", 
                "-in", &user_csr_path,
                "-CA", &ca_cert_path,
                "-CAkey", &ca_key_path,
                "-CAcreateserial",
                "-out", &user_cert_path,
                "-days", &self.user_validity_days.to_string(),
                "-sha256"
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("Failed to sign certificate for user {}", username)
            ));
        }

        Ok(())
    }

    /// Revoke User Certificate
    fn revoke_user_certificate(&self, username: &str) -> io::Result<()> {
        let ca_key_path = format!("{}/ca_private_key.pem", self.ca_dir);
        let ca_cert_path = format!("{}/ca_certificate.pem", self.ca_dir);
        let user_cert_path = format!("{}/{}_certificate.pem", self.users_dir, username);
        let crl_path = format!("{}/ca_crl.pem", self.ca_dir);
        
        // First, verify if certificate exists
        if !Path::new(&user_cert_path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Certificate for user {} not found", username)
            ));
        }

        // Revoke certificate
        let output = Command::new("openssl")
            .args(&[
                "ca", 
                "-revoke", &user_cert_path,
                "-keyfile", &ca_key_path,
                "-cert", &ca_cert_path
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("Failed to revoke certificate for user {}", username)
            ));
        }

        // Generate Certificate Revocation List (CRL)
        let crl_output = Command::new("openssl")
            .args(&[
                "ca", 
                "-gencrl", 
                "-keyfile", &ca_key_path,
                "-cert", &ca_cert_path,
                "-out", &crl_path
            ])
            .output()?;

        if !crl_output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                "Failed to generate Certificate Revocation List"
            ));
        }

        Ok(())
    }

    /// Sign Document/File
    fn sign_document(&self, username: &str, document_path: &str) -> io::Result<()> {
        let user_key_path = format!("{}/{}_private_key.pem", self.users_dir, username);
        let signature_path = format!("{}.sig", document_path);
        
        let output = Command::new("openssl")
            .args(&[
                "dgst", "-sha256", 
                "-sign", &user_key_path,
                "-out", &signature_path,
                document_path
            ])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("Failed to sign document for user {}", username)
            ));
        }

        Ok(())
    }

    /// Verify Document Signature
    fn verify_document_signature(&self, username: &str, document_path: &str) -> io::Result<bool> {
        let user_cert_path = format!("{}/{}_certificate.pem", self.users_dir, username);
        let signature_path = format!("{}.sig", document_path);
        
        let output = Command::new("openssl")
            .args(&[
                "dgst", "-sha256", 
                "-verify", &user_cert_path,
                "-signature", &signature_path,
                document_path
            ])
            .output()?;

        Ok(output.status.success())
    }
}

fn main() -> io::Result<()> {
    let pki_config = PKIConfig::new();

    // Initialize PKI structure
    pki_config.init_pki_structure()?;

    // Generate CA Key and Self-Signed Certificate
    pki_config.generate_ca_key()?;
    pki_config.create_ca_certificate()?;

    // Example user operations
    let test_user = "tudor_popov";
    
    // Generate user key
    pki_config.generate_user_key(test_user)?;
    
    // Generate CSR
    pki_config.generate_csr(test_user)?;
    
    // Sign User Certificate
    pki_config.sign_user_certificate(test_user)?;

    println!("PKI Setup Complete!");

    Ok(())
}