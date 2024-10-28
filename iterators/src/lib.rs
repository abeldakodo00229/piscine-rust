#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let elm = self.v;
        if self.v == 1 || self.v == 0 {
            self.v = 1;
            None
        } else {
            if self.v % 2 == 0 {
                self.v /= 2;
            } else {
                self.v = self.v * 3 + 1;
            }
            Some(Collatz::new(elm))
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n  }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut count = 0;
    let mut value = n;

    while value != 1 {
        if value % 2 == 0 {
            value /= 2;
        } else {
            value = value * 3 + 1;
        }
        count += 1;
    }

    count
}
