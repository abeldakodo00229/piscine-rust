use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32)->Self{
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Error number"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(num: u32)->Self{
        let mut res : Vec<_>  = Vec::new();
        
        if romannum(num).len() == 0{
            res.push(RomanDigit::Nulla);
            return RomanNumber(res);
        }
        for elm in romannum(num).chars(){
             match elm {
                'I' => res.push(RomanDigit::I),
                'V' => res.push(RomanDigit::V),
                'X' => res.push(RomanDigit::X),
                'L' => res.push(RomanDigit::L),
                'C' => res.push(RomanDigit::C),
                'D' => res.push(RomanDigit::D),
                'M' => res.push(RomanDigit::M),
                _ => panic!("Error number")

             }
        }
        
        RomanNumber(res)
    }
}


fn romannum(mut num: u32) -> String{
    let mut res = String::new();
    let digits = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    for (i, digit) in digits.iter().enumerate(){
        while num >= *digit {
            res.push_str(&symbols[i]);
            num -= digit;
        }
    }
    res
          
}
