
pub fn edit_distance(source: &str, target: &str) -> usize {
        if source.len()==0{
            return target.len();
        }else if target.len()==0{
            return source.len();
        }else if &source[0..1] == &target[0..1] {
          return  edit_distance(&source[1..], &target[1..]);
        }else{
            let d1 = edit_distance(&source[1..],target);
            let d2 = edit_distance(&target[1..],source);
            let d3 = edit_distance(&source[1..],&target[1..]);
            let result = if d1<=d2 && d1<=d3 {d1}else if d2<=d1 && d2<=d3 {d2} else {d3};
            return 1 + result;
        }


}


