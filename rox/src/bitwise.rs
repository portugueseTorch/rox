/// returns val as bytes
pub fn get_bytes(val: u32) -> (u8, u8, u8, u8) {
    return (
        (val >> 24 & 0xff) as u8,
        (val >> 16 & 0xff) as u8,
        (val >> 8 & 0xff) as u8,
        (val & 0xff) as u8,
    );
}

/// returns a u32 from an an array of 3 bytes
pub fn u32_from_bytes(bytes: &[u8; 3]) -> u32 {
    u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])
}
