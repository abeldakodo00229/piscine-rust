pub fn add_curry(value: i32) -> impl Fn(i32) -> i32 {
    move |x| x + value
}

