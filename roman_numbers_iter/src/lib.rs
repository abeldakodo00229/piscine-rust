// #[derive(Debug, PartialEq, Eq)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
    CM,
    CD,
    XC,
    XL,
    IX,
    IV,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl RomanNumber {
    pub fn from(mut number: usize) -> RomanNumber {
        let mut roman_digits = vec![];
        let roman_chars = [(1000, RomanDigit::M),
                           (900, RomanDigit::CM),
                           (500, RomanDigit::D),
                           (400, RomanDigit::CD),
                           (100, RomanDigit::C),
                           (90, RomanDigit::XC),
                           (50, RomanDigit::L),
                           (40, RomanDigit::XL),
                           (10, RomanDigit::X),
                           (9, RomanDigit::IX),
                           (5, RomanDigit::V),
                           (4, RomanDigit::IV),
                           (1, RomanDigit::I)];

        for &(value, ref digit) in &roman_chars {
            while number >= value {
                roman_digits.push(digit.clone()); 
                number -= value;
            }
        }

        RomanNumber(roman_digits)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber ;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("{:?}", self.0);
        
        Some(RomanNumber(nombre_suivant_romain(self.0.clone())))
        
    }
}

fn nombre_suivant_romain(nombres_romains: Vec<RomanDigit>) -> Vec<RomanDigit> {
 
    let mut nombre : usize = 0;

    // Convertir les nombres romains en nombre décimal
    for nombre_romain in nombres_romains.iter() {
        match nombre_romain {
            RomanDigit::I => nombre += 1,
            RomanDigit::V => nombre += 5,
            RomanDigit::X => nombre += 10,
            RomanDigit::L => nombre += 50,
            RomanDigit::C => nombre += 100,
            RomanDigit::D => nombre += 500,
            RomanDigit::M => nombre += 1000,
            RomanDigit::CM => nombre += 900,
            RomanDigit::CD => nombre += 400,
            RomanDigit::XC => nombre += 90,
            RomanDigit::XL => nombre += 40,
            RomanDigit::IX => nombre += 9,
            RomanDigit::IV => nombre += 4,
        }
    }
    nombre += 1;
   RomanNumber::from(nombre).0
}
