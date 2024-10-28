pub fn to_url(s: &str) -> String {
    let mut ouput = String::new();
    for letter in s.chars(){
        if letter != ' '{
            ouput.push_str(&letter.to_string());
        }else{
            ouput= ouput+"%20"
        }
    }
    return ouput;
}