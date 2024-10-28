// pub fn is_permutation(s1: &str, s2: &str) -> bool {
// s1.chars().collect::<Vec<_>>().sort()==s2.chars().collect::<Vec<_>>().sort()
// }

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut str1 : Vec<char>   = s1.chars().collect();
    let mut str2 : Vec<char>  = s2.chars().collect();
    str1.sort();
    str2.sort();
    str1==str2
    }
    