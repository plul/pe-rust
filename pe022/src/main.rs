use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let mut names = load_names("names.txt");

    names.sort();

    names
        .into_iter()
        .enumerate()
        .map(|(idx, name)| name_score(&name) * (idx + 1))
        .sum()
}

/// Assumes lower case ascii characters
fn char_score(c: char) -> usize {
    1 + (c as u8 - b'a') as usize
}

fn name_score(name: &str) -> usize {
    name.chars().map(char_score).sum()
}

fn load_names(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("can't open file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong while trying to read file");

    contents
        .to_ascii_lowercase()
        .split(',')
        .map(|s| s.trim_matches('"').to_owned())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn colin() {
        assert_eq!(name_score("colin"), 53);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 871198282);
    }
}
