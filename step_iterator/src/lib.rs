use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T>
where
    T: PartialOrd + Add<Output = T> + Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: PartialOrd + Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }
        let result = self.current;
        self.current = self.current + self.step;
        Some(result)
    }
}
