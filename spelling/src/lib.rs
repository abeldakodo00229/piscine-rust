pub fn spell(n: u64) -> String {
    static ONES: [&str; 20] = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];
    static TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 0 {
        return "zero".to_string();
    } else if n < 20 {
        return ONES[n as usize].to_string();
    } else if n < 100 {
        let ones = n % 10;
        let tens = n / 10;
        if ones == 0 {
            return TENS[tens as usize].to_string();
        } else {
            return format!("{}-{}", TENS[tens as usize], ONES[ones as usize]);
        }
    } else if n < 1000 {
        let ones = n % 100;
        let hundreds = n / 100;
        
        if ones == 0 {
            return format!("{} hundred", ONES[hundreds as usize]);
        } else  {
            return format!(
                "{} hundred {}", ONES[hundreds as usize],
                spell(ones)
            );
        }
    } else if n < 1000000 {
        let ones = n % 1000;
        let thousands = n / 1000;
        if ones == 0 {
            return format!("{} thousand", spell(thousands));
        } else if n / 100 >= 11 {
           
            return format!(
                "{} thousand {} hundred {}",
                spell(thousands),
                spell(ones / 100),
                spell(ones % 100)
            );
        } else {
           
            return format!(
                "{} thousand {}",
                spell(thousands),
                spell(ones % 100)
            );
        }
    } else {
        return "one million".to_string();
    }
}