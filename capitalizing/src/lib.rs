pub fn capitalize_first(input: &str) -> String {
    let mut output = String::new();
    if input.len()==0{
       return  "".to_string();
    }
   output.push_str(&input.chars().next().expect("first_char").to_uppercase().to_string());
    output.push_str(&input[1..]);
    output
}

pub fn title_case(input: &str) -> String {
    let mut output = Vec::new(); 
    let tab : Vec<_> = input.split_whitespace().collect();
    for word in tab {
        output.push(capitalize_first(word))
    }
    output.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut output = String::new();

    for letter in input.chars() {
        if letter.is_uppercase(){
             output.push_str(&letter.to_lowercase().to_string());
        }else   if letter.is_lowercase(){
            output.push_str(&letter.to_uppercase().to_string());
       }else{
        output.push_str(&letter.to_string());
       }
    }
    output
}
