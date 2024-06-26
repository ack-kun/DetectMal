// Normalize the content to it's character "types"
pub fn normalize(content: &[u8]) -> Vec<u8> {
    let mut normalized: Vec<u8> = Vec::new();

    for byte in content {
        match byte {
            0..=31 => normalized.push(b'*'),    // Non visible characters
            32 => normalized.push(b'S'),        // Space
            33..=47 => normalized.push(b'!'),   // Symbols
            48..=57 => normalized.push(b'9'),   // Numbers
            58..=64 => normalized.push(b'='),   // Equation
            65..=90 => normalized.push(b'a'),   // Upper case letters
            91..=96 => normalized.push(b'*'),   // Symbols
            97..=122 => normalized.push(b'a'),  // Lower case letters
            123..=126 => normalized.push(b'>'), // Syntax chars
            127 => normalized.push(b'D'),       // DEL
            128..=254 => normalized.push(b':'), // Extended special
            255 => normalized.push(b'0'),       // Empty
        }
    }

    normalized
}
