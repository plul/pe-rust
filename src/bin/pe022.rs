use std::time::Instant;

const DATA: &[u8] = include_bytes!("pe022-data.txt");

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let data: &str = std::str::from_utf8(DATA).unwrap();
    let mut names = load_names(data);

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

fn load_names(data: &str) -> Vec<String> {
    data.to_ascii_lowercase()
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
