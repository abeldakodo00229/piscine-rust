pub fn delete_and_backspace(s: &mut String) {
    let mut stack = Vec::new();
    let mut skip = 0;

    for c in s.chars() {
        if c == '+' {
            skip += 1;
        } else if c == '-' {
            if !stack.is_empty() {
                stack.pop();
            }
        } else if skip == 0 {
            stack.push(c);
        } else {
            skip -= 1;
        }
    }

    s.clear();
    s.extend(stack.iter().map(|&c| c));
}


pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        let mut parts = equation.split(|c| c == '+' || c == '-').filter(|s| !s.is_empty());
        let mut result = parts.next().unwrap().parse::<i32>().unwrap();

        for part in parts {
            let number: i32 = part.parse().unwrap();
            if equation.contains('+') {
                result += number;
            } else {
                result -= number;
            }
        }

        *equation = result.to_string();
    }
}
