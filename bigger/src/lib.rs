use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
  
    let mut result : i32 =0; 

        for (_,&number) in h.iter(){
            if number > result{
                result = number;
            }
        }   
        result   
}