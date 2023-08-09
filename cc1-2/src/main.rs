use hex;

fn main() {
    let buffer1 = "1c0111001f010100061a024b53535009181c";
    let buffer2 = "686974207468652062756c6c277320657965";
    let xor_bytes = xor_combo(&buffer1, &buffer2);
    let xor_string = hex::encode(xor_bytes);
    println!("{xor_string}");
}

fn xor_combo(buffer1: &str, buffer2: &str) -> Vec<u8> {
    let binary1 = match hex::decode(buffer1) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("Invalid binary input");
            return Vec::new();
        }
    };
    let binary2 = match hex::decode(buffer2) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("Invalid binary input");
            return Vec::new();
        }
    };
    let mut xor_binary: Vec<u8> = Vec::new();
    for (byte1, byte2) in binary1.iter().zip(binary2.iter()) {
        xor_binary.push(byte1 ^ byte2);
    }
    xor_binary
}