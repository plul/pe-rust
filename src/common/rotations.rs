use std::iter;

pub struct Rotations<T: Clone> {
    collection: Vec<T>,
    i: usize,
}

impl<T: Clone> Rotations<T> {
    pub fn new(collection: Vec<T>) -> Self {
        Self { collection, i: 0 }
    }
}

impl<T: Clone> Iterator for Rotations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.collection.len() {
            None
        } else {
            let (left, right) = self.collection.split_at(self.i);
            let rotation: Vec<T> = right.into_iter().chain(left.into_iter()).cloned().collect();

            self.i += 1;
            Some(rotation)
        }
    }
}

impl<T: Clone> iter::FusedIterator for Rotations<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate() {
        let v = vec![1, 2, 3];
        let mut rotations = Rotations::new(v);
        assert_eq!(rotations.next(), Some(vec![1, 2, 3]));
        assert_eq!(rotations.next(), Some(vec![2, 3, 1]));
        assert_eq!(rotations.next(), Some(vec![3, 1, 2]));
        assert_eq!(rotations.next(), None);
        assert_eq!(rotations.next(), None);
    }

}
