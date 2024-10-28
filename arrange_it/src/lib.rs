pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    let mut rearranged_words: Vec<String> = vec![String::new(); words.len()];

    for word in &words {
        for (i, ch) in word.chars().enumerate() {
            if ch.is_numeric() {
                let n: usize = ch.to_digit(10).expect("Error") as usize - 1;
                let mut temp_word = word.to_string();
                temp_word.remove(i); 
                rearranged_words[n] = temp_word;
            }
        }
    }

   
    rearranged_words.join(" ")
}
