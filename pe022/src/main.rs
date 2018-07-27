use std::fs::File;
use std::io::Read;

/// Assumes lower case ascii characters
fn char_score(c: char) -> usize {
    (c as u8 - 96u8) as usize
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

fn main() {
    let mut names = load_names("names.txt");

    names.sort();

    let result: usize = names
        .into_iter()
        .enumerate()
        .map(|(idx, name)| name_score(&name) * (idx + 1))
        .sum();

    println!("{}", result);
}

#[test]
fn colin() {
    assert_eq!(name_score("colin"), 53);
}
