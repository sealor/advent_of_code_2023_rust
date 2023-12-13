#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::{cmp::max, fs, io::Result, sync::OnceLock};

    #[test]
    fn test1() -> Result<()> {
        let content = fs::read_to_string("day-02.txt")?;

        let mut sum = 0;
        for line in content.lines() {
            sum += resolve_game_value_if_possible(line);
        }

        assert_eq!(sum, 2617);
        Ok(())
    }

    fn resolve_game_value_if_possible(line: &str) -> u32 {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX.get_or_init(|| Regex::new("(\\d+) ?(:|red|blue|green)").unwrap());

        let mut game = 0;

        for c in regex.captures_iter(line) {
            if let Some(kind) = c.get(2) {
                if let Some(number) = c.get(1) {
                    let number = number.as_str().parse().expect("Unknown number");

                    match kind.as_str() {
                        ":" => game = number,
                        "red" => {
                            if number > 12 {
                                return 0;
                            }
                        }
                        "green" => {
                            if number > 13 {
                                return 0;
                            }
                        }
                        "blue" => {
                            if number > 14 {
                                return 0;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        game
    }

    #[test]
    fn test2() -> Result<()> {
        let content = fs::read_to_string("day-02.txt")?;

        let mut sum = 0;
        for line in content.lines() {
            sum += resolve_game_value_by_fewest_cubes(line);
        }

        assert_eq!(sum, 59795);
        Ok(())
    }

    fn resolve_game_value_by_fewest_cubes(line: &str) -> u32 {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX.get_or_init(|| Regex::new("(\\d+) (red|blue|green)").unwrap());

        let (mut red, mut green, mut blue) = (0, 0, 0);

        for c in regex.captures_iter(line) {
            if let Some(kind) = c.get(2) {
                if let Some(number) = c.get(1) {
                    let number = number.as_str().parse().expect("Unknown number");

                    match kind.as_str() {
                        "red" => red = max(red, number),
                        "green" => green = max(green, number),
                        "blue" => blue = max(blue, number),
                        color => panic!("Unknown color: {}", color),
                    }
                }
            }
        }
        red * green * blue
    }
}
