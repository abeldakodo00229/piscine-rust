
pub fn pig_latin(text: &str) -> String {
let mut result = String::new();
   if text.len() > 0 {

       if "aeuio".contains(&text[0..1].to_string()){
        result.push_str(text);
        result.push_str("ay");
       }else if text.len() > 2 && &text[1..3].to_string() == &"qu".to_string() {
        result.push_str(&text[3..]);
        result.push_str(&text[0..3]);
        result.push_str("ay");
       }else{
        for (i,c) in text.chars().enumerate(){
            if "aeuio".contains(&c.to_string()){
                result.push_str(&text[i..]);
                result.push_str(&text[0..i]);
                result.push_str("ay");
                break;
            }
        }
       }
   }
    return result;
}



