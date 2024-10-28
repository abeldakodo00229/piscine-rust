pub fn rev_str(input: &str) -> String {
    let mut ouput = String::new();
    for letter in input.chars().rev(){
           ouput.push_str(&letter.to_string())
    }
    return ouput;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
