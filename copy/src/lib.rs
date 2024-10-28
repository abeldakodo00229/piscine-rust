pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let a : f64 = c as f64;
    return (c,a.exp(),a.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();
    let mut count : u32=0;
    for letter in a.chars(){
        count+=1;
        if letter !=' ' {
            // Convertir le caractère en chaîne, puis en f64
            let mut pi: f64 = letter.to_string().parse().unwrap_or(0.0);
            pi =pi.exp();
            // Ajouter le résultat à la chaîne result
            result.push_str(&pi.to_string());
            if count != a.len().try_into().unwrap(){
                result.push(' ');
            }

        }
    }
    return (a, result);
}


pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result: Vec<f64> = Vec::new();

            for n in &b{
            let a : f64 = *n as f64;
             result.push(a.abs().ln());
            }

    return (b, result);
}
