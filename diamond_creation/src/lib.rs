pub fn get_diamond(c: char) -> Vec<String> {
    let c_index = c as usize - 'A' as usize; 
    let diamond_size = c_index * 2 + 1;
 
    let mut diamond = Vec::with_capacity(diamond_size);

   
    for i in 0..=c_index {
        let spaces = c_index - i;
        let letter = (b'A' + i as u8) as char;
        if i == 0 {
            diamond.push(format!("{}{}{}", " ".repeat(spaces), letter," ".repeat(spaces)));
        } else {
            diamond.push(format!("{}{}{}{}{}", " ".repeat(c_index - i), letter, " ".repeat(i*2 -1), letter, " ".repeat(c_index - i)));
        }
    }
  
    for i in (0..c_index).rev() {
        let spaces = c_index - i;
    
        
        let letter = (b'A' + i as u8) as char;
        if i == 0 {
            diamond.push(format!("{}{}{}", " ".repeat(spaces), letter," ".repeat(spaces)));
        } else {
            diamond.push(format!("{}{}{}{}{}", " ".repeat(c_index  - i ), letter, " ".repeat(i*2 -1), letter, " ".repeat(c_index - i)));
        }
    }

    diamond 
}
