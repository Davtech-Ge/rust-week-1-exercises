// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let raw_tx_bytes = match hex::decode(raw_tx_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid hex string".to_string()),
    };

    if raw_tx_bytes.len() < 4 {
        return Err("Transaction too short to contain version".to_string());
    }

    let version = u32::from_le_bytes([
        raw_tx_bytes[0],
        raw_tx_bytes[1],
        raw_tx_bytes[2],
        raw_tx_bytes[3],
    ]);
    println!("{:?}", version);
    Ok(version)
}
