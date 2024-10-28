use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    println!("{}", list.len());
    let result: f64=sum as f64/list.len() as f64;
     result
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut range = list.clone(); 
    
    range.sort(); 
    
    let median_index = list.len() / 2; 
    if list.len() % 2 == 0 {
        return (range[median_index - 1] + range[median_index]) / 2;
    } else {
        return range[median_index];
    }
}


pub fn mode(list: &Vec<i32>) -> i32 {
  let mut n1 : i32 = 0;
  let mut n2 : i32 = 0;
  let mut occurrences_map = HashMap::new();

    for &num in list.iter() {
        let count = occurrences_map.entry(num).or_insert(0);
        *count += 1;
    }
   for (num,n)in  occurrences_map.iter(){
    if n1<*n{
        n1 = *n;
        n2 = *num;
    }
   }
   return n2;
}
