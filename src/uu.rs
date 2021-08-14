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
