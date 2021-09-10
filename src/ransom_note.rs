use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn can_spell(magazine: Vec<char>, note: String) -> bool {
        let mut letters: HashMap<char, usize> = HashMap::new();

        for character in magazine {
            let counter = letters.entry(character).or_insert(0);
            *counter += 1;
        }

        println!("LETTERS {:?}", letters);

        for character in note.chars() {
            if !letters.contains_key(&character) || letters[&character] <= 0 {
                return false;
            }

            *letters.get_mut(&character).unwrap() -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ransom_note() {
        let magazine = vec!['A', 'B', 'C', 'D', 'T', 'A'];
        let note = String::from("CATSDSAS");

        let can_spell = Solution::can_spell(magazine, note);

        println!("Solution is {:?}", can_spell);
    }
}
