use rand::Rng;
use std::collections::HashMap;

pub struct LetterService;

impl LetterService {
    pub fn get_letter(found_letters: &Vec<char>) -> char {
        let mut probabilities: HashMap<char, f32> = HashMap::new();
        for letter in 'A'..='Z' {
            probabilities.insert(letter, 0.1);
        }

        // adds found letters to map
        found_letters.iter().for_each(|l| {
            if let Some(p) = probabilities.get_mut(l) {
                *p += 25.5
            }
        });

        let total_probability: f32 = probabilities.values().sum();
        let rnd_num = rand::rng().random_range(0.0..total_probability);
        let mut cumulative_probability: f32 = 0.0;

        // TODO:(fix) cant return directly from the loop for some reason
        let mut final_letter = ' ';
        for (l, p) in probabilities {
            cumulative_probability += p;
            if rnd_num <= cumulative_probability {
                final_letter = l;
                break;
            }
        }
        final_letter
    }

    pub fn get_gen_letter(number: u16) -> char {
        let mut i = 1;
        let mut letter = ' ';
        for c in 'A'..='Z' {
            if i == number {
                letter = c;
            }
            i += 1;
        }
        letter
    }
}
