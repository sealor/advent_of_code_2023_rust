#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::{cmp::min, collections::HashSet, fs};

    #[test]
    fn test1() {
        let split_regex = Regex::new("[:|]").unwrap();
        let number_regex = Regex::new("\\d+").unwrap();

        let content = fs::read_to_string("day-04.txt").expect("File cannot be read");
        let mut sum = 0;

        for line in content.lines() {
            let card_sections: Vec<&str> = split_regex.splitn(line, 3).collect();
            let (winning_section, own_section) =
                (card_sections.get(1).unwrap(), card_sections.get(2).unwrap());

            let winning_numbers: HashSet<u32> = number_regex
                .find_iter(winning_section)
                .map(|m| m.as_str().parse::<u32>().expect("Unknown number"))
                .collect();

            let own_numbers: HashSet<u32> = number_regex
                .find_iter(own_section)
                .map(|m| m.as_str().parse::<u32>().expect("Unknown number"))
                .collect();

            let winning_count = winning_numbers.intersection(&own_numbers).count() as u32;
            sum += if winning_count > 0 {
                2_i32.pow(winning_count - 1)
            } else {
                0
            };
        }

        assert_eq!(sum, 20667);
    }

    #[test]
    fn test2() {
        let split_regex = Regex::new("[:|]").unwrap();
        let number_regex = Regex::new("\\d+").unwrap();

        let content = fs::read_to_string("day-04.txt").expect("File cannot be read");
        let mut winning_counts: Vec<usize> = Vec::new();

        for line in content.lines() {
            let card_sections: Vec<&str> = split_regex.splitn(line, 3).collect();
            let (winning_section, own_section) =
                (card_sections.get(1).unwrap(), card_sections.get(2).unwrap());

            let winning_numbers: HashSet<u32> = number_regex
                .find_iter(winning_section)
                .map(|m| m.as_str().parse::<u32>().expect("Unknown number"))
                .collect();

            let own_numbers: HashSet<u32> = number_regex
                .find_iter(own_section)
                .map(|m| m.as_str().parse::<u32>().expect("Unknown number"))
                .collect();

            let winning_count = winning_numbers.intersection(&own_numbers).count();
            winning_counts.push(winning_count);
        }

        fn count_cards(winning_counts: &Vec<usize>, row: usize, sum: &mut usize) {
            *sum += 1;
            let count = *winning_counts.get(row).expect("Winning count missing");
            let count = min(count, winning_counts.len());
            for i in 1..=count {
                count_cards(winning_counts, row + i, sum);
            }
        }

        let mut sum = 0;
        for i in 0..winning_counts.len() {
            count_cards(&winning_counts, i, &mut sum);
        }

        assert_eq!(sum, 5833065);
    }
}
