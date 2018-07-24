extern crate shared;

use shared::util::is_even;

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
        if let Some(len) = self.known.get(n) {
            if *len > 0 {
                return *len;
            }
        }

        // Produce the next integer in the sequence.
        let m = match is_even(n) {
            true => n / 2,
            false => 3 * n + 1,
        };

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
    let n = 1_000_000;

    // Some numbers will go over one million, but we will just cache the ones below.
    let mut chain_inspector = ChainInspector::with_memory(n);

    let (start, _) = (1..n)
        .map(|start| (start, chain_inspector.inspect(start)))
        .max_by_key(|(_, chain_len)| *chain_len)
        .unwrap();

    println!("{}", start);
}
