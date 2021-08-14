pub fn encode (decoded: &str) -> String {
    let mut encoded = String::from((decoded.len() as u8 + 32) as char);
    let mut padded: Vec<u8> = decoded.as_bytes().to_vec();

    padded.extend(std::iter::repeat(0).take((3 - (decoded.len() % 3)) % 3));

    for c in padded.chunks(3) {
        encoded.push((32 + ((c[0] & 0b11111100) >> 2)) as char);
        encoded.push((32 + (((c[0] & 0b00000011) << 4) | ((c[1] & 0b11110000) >> 4))) as char);
        encoded.push((32 + (((c[1] & 0b00001111) << 2) | ((c[2] & 0b11000000) >> 6))) as char);
        encoded.push((32 + (c[2] & 0b00111111)) as char);
    }

    encoded.replace(" ", "`")
}

pub fn decode (encoded: &str) -> String {
    let mut decoded = String::new();
    let mapped: Vec<u8> = encoded.bytes().map(|b| (b - 32) % 64).collect();
    let length = mapped[0] as usize;

    for c in mapped[1..].chunks(4) {
        decoded.push((((c[0] & 0b00111111) << 2) | ((c[1] & 0b00110000) >> 4)) as char);
        decoded.push((((c[1] & 0b00001111) << 4) | ((c[2] & 0b00111100) >> 2)) as char);
        decoded.push((((c[2] & 0b00000011) << 6) | ((c[3] & 0b00111111) >> 0)) as char);
    }

    // Remove excess empty bytes
    decoded.truncate(length);

    decoded
}
