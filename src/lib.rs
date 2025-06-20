pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_hex = &raw_tx_hex[..8];

    let bytes = (0..8)
        .step_by(2)
        .map(|i| u8::from_str_radix(&version_hex[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| "Hex decode error".to_string())?;

    if bytes.len() != 4 {
        return Err("Invalid version length".to_string());
    }

    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)
}
