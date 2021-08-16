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

pub fn encode (clear: &str) -> String {
    let mut encoded = String::from((clear.len() as u8 + 32) as char);
    let mut padded: Vec<u8> = clear.as_bytes().to_vec();

    padded.extend(std::iter::repeat(0).take((3 - (clear.len() % 3)) % 3));

    for chunk in padded.chunks(3) {
        encoded.push(CHARSET[usize::from((chunk[0] & 0b11111100) >> 2)]);
        encoded.push(CHARSET[usize::from(((chunk[0] & 0b00000011) << 4) | ((chunk[1] & 0b11110000) >> 4))]);
        encoded.push(CHARSET[usize::from(((chunk[1] & 0b00001111) << 2) | ((chunk[2] & 0b11000000) >> 6))]);
        encoded.push(CHARSET[usize::from(chunk[2] & 0b00111111)]);
    }

    encoded.replace(" ", "`")
}

pub fn decode (encoded: &str) -> String {
    let mut decoded = String::new();
    let mapped: Vec<u8> = encoded.bytes().map(|b| (b - 32) % 64).collect();
    let length = mapped[0] as usize;

    for chunk in mapped[1..].chunks(4) {
        decoded.push(char::from(((chunk[0] & 0b00111111) << 2) | ((chunk[1] & 0b00110000) >> 4)));
        decoded.push(char::from(((chunk[1] & 0b00001111) << 4) | ((chunk[2] & 0b00111100) >> 2)));
        decoded.push(char::from(((chunk[2] & 0b00000011) << 6) | ((chunk[3] & 0b00111111) >> 0)));
    }

    // Remove excess empty bytes
    decoded.truncate(length);

    decoded
}
