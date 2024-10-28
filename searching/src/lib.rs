pub fn search(array: &[i32], key: i32) -> Option<usize> {

    for (i,&elm) in array.iter().enumerate() {
        if elm == key{
        return Some(i)
        }
    }
    None
}