pub use case::CaseExt;
pub use std::cmp::min;
pub use std::cmp::max;

pub fn expected_variable(input: &str, expected: &str) -> Option<String> {
    let input1 = input.to_lowercase();
    let input2 = expected.to_lowercase();

    if input == "" || input.contains(" ") || input.contains("-") {
        return None;
    }
  

    if input1 == input1.to_camel() || input1 == input1.to_snake() {
        let distance = edit_distance(&input1, &input2) as f64;
        let similarity_percentage = ((1.0 - (distance /  input1.len().max(input2.len()) as f64)) * 100.0) as f64;

        if similarity_percentage >= 50.0 {
        return   Some( similarity_percentage.round().to_string() +"%");
        } 
            return None;
        
    } 
   None

}


pub fn edit_distance(source: &str, target: &str) -> usize {
    let n = source.len();
    let m = target.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
            }
        }
    }

    dp[n][m]
}


