pub fn is_pangram(s: &str) -> bool {
 
    for alpha_letter in 'a'..='z'{
      if !s.to_lowercase().contains(&alpha_letter.to_string()){
        return false;
      }
    }
    return true;
}
