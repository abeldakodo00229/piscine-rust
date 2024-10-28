pub fn number_logic(num: u32) -> bool {
let mut res : usize =0;
let chaine : String = num.to_string(); 
      for c in chaine.chars(){
        let n : usize = c.to_string().parse().expect("erro");
        res += n.pow(chaine.len().try_into().unwrap());
      } 
      if res == num.try_into().unwrap() {
        return true;
      }
      return false;
}