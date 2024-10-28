pub fn first_fifty_even_square() -> Vec<i32> {
let mut result : Vec<i32> = Vec::new();
let mut n : i32 = 2;
        while result.len() < 50{
                result.push(n.pow(2));
                n+=2;
        }
        result
}