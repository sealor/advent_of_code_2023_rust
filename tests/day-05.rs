#[cfg(test)]
mod tests {
    use std::{
        cmp::{max, min},
        fs,
    };

    #[derive(Default)]
    struct Mapping {
        start: i64,
        end: i64,
        diff: i64,
    }

    impl Mapping {
        fn new(start: i64, end: i64, diff: i64) -> Self {
            Self { start, end, diff }
        }
    }

    #[derive(Default)]
    struct CategoryMap {
        mappings: Vec<Mapping>,
        next: Box<Option<CategoryMap>>,
    }

    impl CategoryMap {
        fn resolve_location(&self, number: i64) -> i64 {
            let default = Mapping::default();
            let mapping = self
                .mappings
                .iter()
                .find(|range| range.start <= number && number < range.end)
                .unwrap_or(&default);

            let mapped_number = number + mapping.diff;

            match self.next.as_ref() {
                Some(next) => next.resolve_location(mapped_number),
                None => mapped_number,
            }
        }

        fn resolve_min_location_by_range(&self, start: i64, end: i64) -> i64 {
            let overlapping: Vec<&Mapping> = self
                .mappings
                .iter()
                .skip_while(|m| m.end <= start)
                .take_while(|m| m.start < end)
                .collect();

            let mut min_location = i64::MAX;
            for mapping in overlapping {
                let new_start = max(mapping.start, start) + mapping.diff;
                let new_end = min(mapping.end, end) + mapping.diff;

                let location = match self.next.as_ref() {
                    Some(next) => next.resolve_min_location_by_range(new_start, new_end),
                    None => new_start,
                };
                min_location = min(min_location, location);
            }
            min_location
        }
    }

    #[test]
    fn test1() {
        let content = fs::read_to_string("day-05.txt").expect("File cannot be read");

        let mut lines = content.lines();
        let seeds: Vec<i64> = lines
            .next()
            .expect("First line is empty")
            .split_terminator(' ')
            .skip(1)
            .map(|num| num.parse().expect("No valid number"))
            .collect();

        let mut anchor = CategoryMap::default();
        let mut last = &mut anchor;
        for line in lines {
            let first_char = line.chars().next();
            let Some(c) = first_char else {
                continue;
            };

            if c.is_alphabetic() {
                last.mappings.sort_by(|a, b| a.start.cmp(&b.start));

                let new = CategoryMap::default();
                last.next = Box::new(Some(new));
                last = (*last.next).as_mut().unwrap();
            } else if c.is_numeric() {
                let numbers: Vec<i64> = line
                    .split_terminator(' ')
                    .map(|num| num.parse().expect("No valid number"))
                    .collect();
                last.mappings.push(Mapping::new(
                    numbers[1],
                    numbers[1] + numbers[2],
                    numbers[0] - numbers[1],
                ));
            }
        }

        last.mappings.sort_by(|a, b| a.start.cmp(&b.start));

        let mut min_seed = i64::MAX;
        for mut seed in seeds {
            seed = anchor.resolve_location(seed);
            min_seed = min(min_seed, seed);
        }

        assert_eq!(min_seed, 836040384);
    }

    #[test]
    fn test2() {
        let content = fs::read_to_string("day-05.txt").expect("File cannot be read");

        let mut lines = content.lines();
        let seeds: Vec<i64> = lines
            .next()
            .expect("First line is empty")
            .split_terminator(' ')
            .skip(1)
            .map(|num| num.parse().expect("No valid number"))
            .collect();
        let seed_starts: Vec<i64> = seeds.iter().step_by(2).cloned().collect();
        let seed_ranges: Vec<i64> = seeds.iter().skip(1).step_by(2).cloned().collect();

        let mut first_map: Option<CategoryMap> = None;
        let mut current_map: &mut Option<CategoryMap> = &mut None;
        for line in lines {
            let first_char = line.chars().next();
            let Some(first_char) = first_char else {
                continue;
            };

            if line.ends_with("map:") {
                let new_map = CategoryMap::default();

                if let Some(last_map) = current_map.as_mut() {
                    prepare_mapping(last_map);
                    last_map.next = Box::new(Some(new_map));
                    current_map = last_map.next.as_mut();
                } else {
                    first_map = Some(new_map);
                    current_map = &mut first_map;
                };

                continue;
            }

            if first_char.is_numeric() {
                let numbers: Vec<i64> = line
                    .split_terminator(' ')
                    .map(|num| num.parse().expect("No valid number"))
                    .collect();

                let last_map = current_map.as_mut().expect("No mapping in almanac found");
                last_map.mappings.push(Mapping::new(
                    numbers[1],
                    numbers[1] + numbers[2],
                    numbers[0] - numbers[1],
                ));
            }
        }

        let last_map = current_map.as_mut().expect("No mapping in almanac found");
        prepare_mapping(last_map);

        let first_map = first_map.expect("No mapping in almanac found");

        let mut min_location = i64::MAX;
        for i in 0..seed_starts.len() {
            let seed_start = seed_starts.get(i).unwrap();
            let seed_range = seed_ranges.get(i).unwrap();
            let seed_end = seed_start + seed_range;

            let location = first_map.resolve_min_location_by_range(*seed_start, seed_end);
            min_location = min(min_location, location);
        }

        assert_eq!(min_location, 10834440);
    }

    fn prepare_mapping(map: &mut CategoryMap) {
        map.mappings.sort_by(|a, b| a.start.cmp(&b.start));

        // fill gaps from beginning
        let mut missing_mappings = Vec::new();
        let mut prev_end = i64::MIN;
        for m in &map.mappings {
            if m.start > prev_end {
                missing_mappings.push(Mapping::new(prev_end, m.start, 0));
            }
            prev_end = m.end;
        }
        map.mappings.extend(missing_mappings);
        map.mappings.sort_by(|a, b| a.start.cmp(&b.start));

        // add one mapping if nothing else is there
        if map.mappings.is_empty() {
            map.mappings.push(Mapping::new(i64::MIN, i64::MAX, 0));
        }

        // add final mapping if needed
        if let Some(last) = map.mappings.last() {
            if last.end < i64::MAX {
                map.mappings.push(Mapping::new(last.end, i64::MAX, 0));
            }
        }
    }
}
