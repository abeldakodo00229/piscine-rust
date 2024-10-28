pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
  let vec : Vec<&str> = s.split_whitespace().collect();
  let mut res : Vec<u32> = Vec::new();
  for elm in vec {
      if &elm[elm.len()-1..elm.len()] == "k"{
        // println!("ee");
          let n : f64 = elm[0..elm.len()-1].parse::<f64>().expect("Error converting") * 1000.0;
          res.push(n as u32);
        }else{
            let n : f64 = elm.parse::<f64>().expect("Error converting");
            res.push(n as u32);
        }
    }
    let result : Box<Vec<u32>> = Box::new(res);

  return result;
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    let res : Vec<u32> = a.to_vec();
         res
}