extern crate base64;
use hex;

use std::io;


fn main() {
    let hex_str = String::from(get_user_input());
    let base64_str = hex_to_base64(&hex_str);
    println!("Base64: {}", base64_str);
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn hex_to_base64(hex_str: &str) -> String {
    // Decode the hex string into a byte array
    let decoded_bytes = match hex::decode(hex_str) {
        Ok(bytes) => bytes,
        Err(error) => return String::from(format!("{}",error)),
    };
    
    // Encode the byte array into base64
    base64::encode(decoded_bytes)
    // decoded_bytes
}