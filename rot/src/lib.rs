pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    // println!("{}",b'z');
    let mut  n: i8 = key; 
    if n<0 {
        n =n + 26;
    }
    for alpha in input.chars() {
        match alpha{
            'a'..='z' => res.push(((alpha as u8 - b'a'  + n as u8)%26 + b'a')  as char),
            'A'..='Z' => res.push(((alpha as u8 - b'A'  + n as u8)%26 + b'A') as char),
                _ => res.push(alpha),
        }
    }
    return res;
}