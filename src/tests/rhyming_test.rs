#[cfg(test)]
mod tests {
    use crate::{struct_parser::parse_struct, tests::rhyming_test::is_aabb_rhyming_scheme};

    #[test]
    fn test_rhyming_1() {
        let input = "A **Hero** begins his journey in the land,
                           and his *name* is _wordly_, as I understand,
                           at the unknown *age*, but _countable_,
                           and he should sit now at the table.";

        let is_aabbccdd = is_aabb_rhyming_scheme(input);

        assert_eq!(is_aabbccdd, true);
    }

    #[test]
    fn test_rhyming_2() {
        let input = "The **Enemy** is a creature of the dark,
                           and his *health* is _countable_, but not a mark.";

        let is_aabbccdd = is_aabb_rhyming_scheme(input);

        assert_eq!(is_aabbccdd, true);
    }
}

use hypher::hyphenate;
use hypher::Lang;
use ttaw;

pub fn is_aabb_rhyming_scheme(poem: &str) -> bool {
    let lines: Vec<&str> = poem.lines().collect();

    let lines = lines
        .iter()
        .map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .flatten()
        .collect::<Vec<&str>>();

    if lines.len() % 2 != 0 {
        // The poem should have an even number of lines for AABBCCDD rhyming scheme
        panic!(
            "The poem does not have an even number of lines!: {}",
            lines.len(),
        );
    }

    for i in (0..lines.len()).step_by(2) {
        let line1 = lines[i].trim();
        let line2 = lines[i + 1].trim();

        //Drop non-alphanumeric characters
        let line1 = line1
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        let line2 = line2
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();

        let words1: Vec<&str> = line1.split_whitespace().collect();
        let words2: Vec<&str> = line2.split_whitespace().collect();

        let syllables1: Vec<&str> = words1
            .iter()
            .map(|word| hyphenate(word, Lang::English))
            .flatten()
            .collect();

        let syllables2: Vec<&str> = words2
            .iter()
            .map(|word| hyphenate(word, Lang::English))
            .flatten()
            .collect();

        // if syllables1.len() != syllables2.len() {
        //     panic!(
        //         "Syllable count mismatch of line {} and {}, {} and {}",
        //         i,
        //         i + 1,
        //         syllables1.len(),
        //         syllables2.len()
        //     );
        // }

        let last_syllable1 = syllables1.last().unwrap();
        let last_syllable2 = syllables2.last().unwrap();

        let cmudict = ttaw::cmu::CmuDict::new("cmudict.json").unwrap();

        let is_rhyming = cmudict.rhyme(last_syllable1, last_syllable2).unwrap();

        if !is_rhyming {
            panic!(
                "Line {} and {} are not rhyming!: {}, {}",
                i,
                i + 1,
                line1,
                line2
            );
        }
    }

    true
}
