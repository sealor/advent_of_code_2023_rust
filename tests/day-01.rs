#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::{
        collections::HashMap,
        fs::File,
        io::{Read, Result},
        string::String,
        sync::OnceLock,
    };

    fn get_number_map() -> &'static HashMap<&'static str, u32> {
        static NUMBER_MAP: OnceLock<HashMap<&str, u32>> = OnceLock::new();
        NUMBER_MAP.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert("one", 1);
            map.insert("two", 2);
            map.insert("three", 3);
            map.insert("four", 4);
            map.insert("five", 5);
            map.insert("six", 6);
            map.insert("seven", 7);
            map.insert("eight", 8);
            map.insert("nine", 9);
            map
        })
    }

    #[test]
    fn test1() -> Result<()> {
        let mut file = File::open("day-01.txt")?;
        let mut content = String::new();
        let _ = file.read_to_string(&mut content)?;

        let mut sum = 0;
        for line in content.lines() {
            let mut iter = line.matches(char::is_numeric);

            let lower = iter.next().expect("No number found");
            let higher = iter.last().unwrap_or(lower);
            let number = format!("{}{}", lower, higher);

            sum += number.parse::<u32>().unwrap();
        }

        assert_eq!(56108, sum);
        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let mut file = File::open("day-01.txt")?;
        let mut content = String::new();
        let _ = file.read_to_string(&mut content)?;

        let pattern = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        let mut sum = 0;
        for line in content.lines() {
            let mut iter = RollingRegexIter::new(pattern.clone(), line);

            let lower = iter.next().expect("No number found");
            let higher = iter.last().unwrap_or(lower);

            let number = format!("{}{}", lower, higher);
            sum += number.parse::<u32>().unwrap();
        }

        assert_eq!(55652, sum);
        Ok(())
    }

    fn parse(number: &str) -> u32 {
        number
            .parse::<u32>()
            .unwrap_or_else(|_e| *get_number_map().get(number).unwrap())
    }

    struct RollingRegexIter {
        regex: Regex,
        text: String,
        start: usize,
    }

    impl RollingRegexIter {
        fn new(regex: Regex, text: &str) -> Self {
            RollingRegexIter {
                regex,
                text: text.to_string(),
                start: 0,
            }
        }
    }

    impl Iterator for RollingRegexIter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            match self.regex.find_at(&self.text, self.start) {
                Some(m) => {
                    self.start = m.start() + 1;
                    Some(parse(m.as_str()))
                }
                None => None,
            }
        }
    }
}
