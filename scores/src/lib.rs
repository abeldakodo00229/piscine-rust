pub fn score(chaine: &str) -> u64{
      let mut res : u64 = 0;
    for elm in chaine.chars() {

      let  n = match elm.to_ascii_uppercase(){
          'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T'	 =>1,
          'D'|'G' =>2,
          'B'|'C'|'M'|'P' =>3,
          'F'|'H'|'V' |'W'|'Y' =>4,
          'K'	 =>5,
          'J'|'X'	 =>8,
          'Q'|'Z' => 10,
            _ => 0,
        };
        // println!("{}",n);
        res += n;

    }
    res
}