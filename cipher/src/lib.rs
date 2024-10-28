#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
   pub validation: bool,
   pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {

    if original.trim().is_empty() || ciphered.trim().is_empty() {
        return None; 
    }

    let mut atbash = String::new();

    for c in original.chars() {
        match c {
            'A'..='Z' => atbash.push((b'Z' - c as u8 + b'A') as char),
            'a'..='z' => atbash.push((b'z' - c as u8 + b'a') as char),
            _ => atbash.push(c),
        }
    }

    if atbash == ciphered.to_string() {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, atbash)))
    }
}
