use num::One;
use std::mem;
use std::ops::Add;

pub struct FibonacciIterator<T: One>(T, T)
where
    for<'a> &'a T: Add<Output = T>;

impl<T: One> FibonacciIterator<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    pub fn new() -> FibonacciIterator<T> {
        FibonacciIterator(T::one(), T::one())
    }
}

impl<T: One> Iterator for FibonacciIterator<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let f = &self.0 + &self.1;
        let f = mem::replace(&mut self.1, f);
        let f = mem::replace(&mut self.0, f);
        Some(f)
    }
}
