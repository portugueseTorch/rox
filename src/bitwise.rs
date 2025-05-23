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

/// computes the difference in bytes between dst and src (cur - start)
/// start must be larger than src, as the value is returned in usize
#[macro_export]
macro_rules! ptr_offset {
    ($start:expr, $cur:expr) => {{
        let offset: isize = $cur.byte_offset_from($start);
        usize::try_from(offset).expect("pointer offset must be non-negative")
    }};
}
