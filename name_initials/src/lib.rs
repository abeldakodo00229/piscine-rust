pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut strr = String::new();
    let mut n: u32 =0;
    for elm in names {
        for letter in elm.chars() {
            if n==0{
              strr.push(letter);
              strr.push('.');
              strr.push(' ');
                n=1;
          }else if n==2{
            strr.push(letter);
            strr.push('.');
            n=1
          }
          if letter==' '{
            n=2;
          }
        }
        // println!("{}",strr);
        result.push(strr);
        strr=String::new();
                n=0;
    }
    return result;
}

// pub fn refactor(s: String) -> String {
//     let mut strr = String::new();
//     let mut i: u32 = 0;

//     for letter in s.chars() {
//         if letter == ' ' {
//             i = 1;
//             if i == 1 {
//                 strr.push('.');
//             }
//             strr.push(' ');
//         }
//         strr.push(letter);
//     }

//     strr.push('.'); 
//     strr
// }