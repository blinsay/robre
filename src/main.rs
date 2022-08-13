use std::io::stdin;

fn main() {
    let pattern = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            eprintln!("usage: pattern");
            std::process::exit(1);
        }
    };

    for line in stdin().lines() {
        let line = line.unwrap();
        if is_match(&pattern, &line) {
            println!("{}", line);
        }
    }
}

fn is_match(regexp: &str, text: &str) -> bool {
    let regexp: Vec<char> = regexp.chars().collect();
    let text: Vec<char> = text.chars().collect();

    if Some(&'^') == regexp.first() {
        return is_match_here(&regexp[1..], &text);
    }

    for i in 0..=text.len() {
        if is_match_here(&regexp, &text[i..]) {
            return true;
        }
    }
    false
}

fn is_match_here(regexp: &[char], text: &[char]) -> bool {
    match regexp {
        [] => true,
        ['$'] => text.is_empty(),
        [c, '*', regexp @ ..] => is_star_match(*c, regexp, text),
        [c, regexp @ ..] if !text.is_empty() && (*c == '.' || *c == *text.first().unwrap()) => {
            is_match_here(regexp, &text[1..])
        }
        _ => false,
    }
}

fn is_star_match(c: char, regexp: &[char], text: &[char]) -> bool {
    let mut text = text;
    loop {
        if is_match_here(regexp, text) {
            return true;
        }
        if text.is_empty() || !(c == '.' || c == *text.first().unwrap()) {
            break;
        }
        text = &text[1..];
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_patterns() {
        let tcs = vec![
            ("^$", "", true),
            ("$", "", true),
            ("^$", "anything", false),
            (".*", "foobar", true),
            ("a", "foobar", true),
            ("c", "foobar", false),
            ("c", "foobar", false),
            ("foo.*", "foobar", true),
            ("foo.*r", "foobar", true),
            ("o*bar", "foobar", true),
            ("o*baz", "foobar", false),
        ];

        let mut failures = vec![];
        for (pat, text, expected) in tcs {
            let actual = is_match(pat, text);
            if actual != expected {
                failures.push((pat, text, expected, actual));
            }
        }

        assert!(
            failures.is_empty(),
            "{} patterns did not match as expected: {:?}",
            failures.len(),
            failures
        );
    }
}
