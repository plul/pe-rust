use std::iter;

pub struct Permutations<T: Clone> {
    collection: Vec<T>,
    p: usize,
    possibilities: usize,
}

impl<T: Clone> Permutations<T> {
    pub fn new(collection: Vec<T>) -> Permutations<T> {
        let possibilities = Self::factorial(collection.len());
        Permutations {
            collection,
            p: 0,
            possibilities,
        }
    }

    pub fn get_permutation(&self, mut target: usize) -> Vec<T> {
        assert!(target < self.possibilities);

        let mut c = self.collection.clone();
        let mut v = Vec::<T>::with_capacity(c.len());

        while !c.is_empty() {
            let l = c.len();
            let f = Self::factorial(l);

            let i = l * target / f;
            v.push(c.remove(i));

            target %= f / l;
        }

        v
    }

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p == self.possibilities {
            None
        } else {
            let permutation = self.get_permutation(self.p);
            self.p += 1;
            Some(permutation)
        }
    }
}

impl<T: Clone> iter::FusedIterator for Permutations<T> {}
