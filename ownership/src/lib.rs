pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();
    for letter in s.chars(){
        if (letter.is_uppercase() && result.len()!=0) || letter=='_'{
        break;
        }
        result.push(letter);
    }
    s=result;
    return  s;
}