pub fn scytale_cipher(message: String, size: u32) -> String {

    let message_len = message.len() as usize;
    let size = size as usize;
    let number_words = (message_len + size - 1) / size;

    let mut result = String::new();

    for col in 0..size {
        for row in 0..number_words {
            let index = row * size + col;
            if index < message_len {
                result.push(message.chars().nth(index).unwrap());
            } else {
                result.push(' ');
            }
        }
    }

    result.trim().to_string()
}

