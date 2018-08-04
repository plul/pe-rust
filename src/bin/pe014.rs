use std::time::Instant;

struct ChainInspector {
    known: Vec<usize>,
}

impl ChainInspector {
    fn with_memory(memory_size: usize) -> ChainInspector {
        ChainInspector {
            known: vec![0; memory_size],
        }
    }

    /// Return the length of the chain produced by n
    fn inspect(&mut self, n: usize) -> usize {
        if n == 1 {
            return 1;
        }

        // If the result is already known, return early.
        if let Some(&len) = self.known.get(n) {
            if len > 0 {
                return len;
            }
        }

        // Produce the next integer in the sequence.
        let m = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };

        // Find the length of the chain recursively.
        let len = self.inspect(m) + 1;

        // Remember the result.
        if let Some(elem) = self.known.get_mut(n) {
            *elem = len;
        }

        len
    }
}

fn main() {
    let t_0 = Instant::now();
    let result = problem(1_000_000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    // Some numbers will go over `n`, but we will just cache the ones below.
    let mut chain_inspector = ChainInspector::with_memory(n);

    (1..n)
        .map(|start| (start, chain_inspector.inspect(start)))
        .max_by_key(|(_, chain_len)| *chain_len)
        .map(|(start, _)| start)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(1_000_000), 837799);
    }
}
