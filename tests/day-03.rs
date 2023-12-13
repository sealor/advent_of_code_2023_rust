#[cfg(test)]
mod tests {
    use std::{cmp::min, fs, sync::OnceLock};

    use regex::Regex;

    #[test]
    fn test1() {
        let content = fs::read_to_string("day-03.txt").expect("File cannot be read");
        let mut lines: Vec<&str> = content.split_terminator('\n').collect();

        let mut sum = 0;

        let empty_line = lines[0].replace(|_| true, ".");
        lines.insert(0, &empty_line);
        lines.push(&empty_line);

        let mut curr_line: &str = lines[0];
        let mut next_line: &str = lines[1];
        for i in 1..(lines.len() - 1) {
            let prev_line = curr_line;
            curr_line = next_line;
            next_line = lines[i + 1];

            static REGEX: OnceLock<Regex> = OnceLock::new();
            let number_regex = REGEX.get_or_init(|| Regex::new("\\d+").unwrap());

            for m in number_regex.find_iter(curr_line) {
                if matches_symbol(prev_line, m.start(), m.end())
                    || matches_symbol(next_line, m.start(), m.end())
                    || matches_symbol(curr_line, m.start(), m.start())
                    || matches_symbol(curr_line, m.end(), m.end())
                {
                    let number: u32 = m.as_str().parse().expect("Unknown number");
                    sum += number;
                }
            }
        }

        assert_eq!(sum, 556057);
    }

    fn matches_symbol(line: &str, start: usize, end: usize) -> bool {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let symbol_regex = REGEX.get_or_init(|| Regex::new("[^0-9.]").unwrap());

        let start = start.saturating_sub(1);
        let end = min(end + 1, line.len());

        symbol_regex.is_match(&line[start..end])
    }

    #[test]
    fn test2() {
        let content = fs::read_to_string("day-03.txt").expect("File cannot be read");
        let mut lines: Vec<&str> = content.split_terminator('\n').collect();

        let mut sum = 0;

        let empty_line = lines[0].replace(|_| true, ".");
        lines.insert(0, &empty_line);
        lines.push(&empty_line);

        let mut curr_line: &str = lines[0];
        let mut next_line: &str = lines[1];
        for i in 1..(lines.len() - 1) {
            let prev_line = curr_line;
            curr_line = next_line;
            next_line = lines[i + 1];

            static REGEX: OnceLock<Regex> = OnceLock::new();
            let gear_regex = REGEX.get_or_init(|| Regex::new("\\*").unwrap());

            for m in gear_regex.find_iter(curr_line) {
                let mut gears: Vec<&str> = Vec::new();

                static REGEX: OnceLock<Regex> = OnceLock::new();
                let number_regex = REGEX.get_or_init(|| Regex::new("\\d+").unwrap());

                for n in number_regex.find_iter(prev_line) {
                    if n.start().saturating_sub(1) <= m.start() && m.end() <= n.end() + 1 {
                        gears.push(n.as_str());
                    }
                }
                for n in number_regex.find_iter(curr_line) {
                    if m.start() == n.end() || m.end() == n.start() {
                        gears.push(n.as_str());
                    }
                }
                for n in number_regex.find_iter(next_line) {
                    if n.start().saturating_sub(1) <= m.start() && m.end() <= n.end() + 1 {
                        gears.push(n.as_str());
                    }
                }

                if gears.len() == 2 {
                    let gear1: u32 = gears[0].parse().expect("Unknown number");
                    let gear2: u32 = gears[1].parse().expect("Unknown number");
                    sum += gear1 * gear2;
                }
            }
        }

        assert_eq!(sum, 82824352);
    }
}
