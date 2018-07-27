//! This is a kind of search algorithm - it does not generate all possible permutations.
//!
//! There will be an equal number of permutations starting with any given character,
//! So if the input is {0, 1, 2}, then there are equally many permutations arising
//! from that set that have 0, 1 or 2 as the first digit.
//! Furthermore, those are all ordered, so it is fairly simple to figure out which
//! character should come first, and then the same logic can be applied for the next...

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn main() {
    let mut chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut target = 1_000_000 - 1;

    let mut permutation = Vec::<char>::new();
    while chars.len() > 0 {
        let l = chars.len();
        let f = factorial(l);

        let i = l * target / f;
        permutation.push(chars.remove(i));

        target %= f / l;
    }

    let result: String = permutation.into_iter().collect();

    println!("{}", result);
}
