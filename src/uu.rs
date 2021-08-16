pub const CHARSET: [char; 64] = [
    '`', '!', '"', '#', '$', '%', '&', '\'',
    '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', ':', ';', '<', '=', '>', '?',
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
    'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
    'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
];

pub fn encode (clear: &[u8]) -> String {
    let mut encoded = String::from((clear.len() as u8 + 32) as char);
    let mut padded: Vec<u8> = clear.to_vec();
    let padding = (3 - (clear.len() % 3)) % 3;

    // Pad with 0s
    padded.extend(std::iter::repeat(0).take(padding));

    for chunk in padded.chunks(3) {
        let bits = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32);
        encoded.push(encode_value((bits >> 18) as u8 & 0b111111));
        encoded.push(encode_value((bits >> 12) as u8 & 0b111111));
        encoded.push(encode_value((bits >> 6) as u8 & 0b111111));
        encoded.push(encode_value((bits >> 0) as u8 & 0b111111));
    }

    // Replace spaces by backtick
    encoded.replace(" ", "`")
}

pub fn decode (encoded: &str) -> String {
    let mut decoded = String::new();
    let mapped: Vec<u8> = encoded.chars().map(|c| decode_value(c)).collect();
    let length = mapped[0] as usize;

    for chunk in mapped[1..].chunks(4) {
        let bits = (chunk[0] as u32) << 18 | (chunk[1] as u32) << 12 | (chunk[2] as u32) << 6 | (chunk[3] as u32);
        decoded.push((bits >> 16) as u8 as char);
        decoded.push((bits >> 8) as u8 as char);
        decoded.push((bits >> 0) as u8 as char);
    }

    // Remove excess empty bytes
    decoded.truncate(length);

    decoded
}

pub fn encode_value (value: u8) -> char {
    CHARSET[value as usize]
}

pub fn decode_value (value: char) -> u8 {
    (value as u8 - 32) % 64
}
