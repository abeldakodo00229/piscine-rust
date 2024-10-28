pub fn talking(text: &str) -> &str {

        if text.trim().is_empty() {
        return "Just say something!";
        } else if is_maj(text)  && text[text.len() -1..].contains("?"){
            return "Quiet, I am thinking!";  
        }else if !is_maj(text)  && text[text.len() -1..].contains("?"){
           return "Sure."
        }else if is_maj(text){
            return "There is no need to yell, calm down!";
        }
        "Interesting"
}


pub fn is_maj(text: &str) -> bool {
    let mut n : usize = 0;
    for elm in text.chars() {
        if elm.is_alphabetic() && !elm.is_ascii_uppercase() {
            return false;
        }
        if !elm.is_alphabetic(){
             n += 1;
        }
    }
    if n == text.len(){
        return false;
    }
    true
}
