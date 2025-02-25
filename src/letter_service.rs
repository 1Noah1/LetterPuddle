use colored::Color;
use rand::Rng;
use std::collections::HashMap;

pub struct LetterService;

impl LetterService {
    pub fn get_letter(found_letters: &Vec<char>) -> char {
        let mut probabilities: HashMap<char, f32> = HashMap::new();
        for letter in 'A'..='F' {
            probabilities.insert(letter, 0.1);
        }

        // adds found letters to map
        found_letters.iter().for_each(|l| {
            if let Some(p) = probabilities.get_mut(l) {
                *p += 15.0
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

    pub fn get_gen_letter(number: u32) -> char {
        let mut num = number.clone();
        let mut i = 1;
        let mut letter = ' ';
        if num > 26 {
            num = num - 26;
        }
        for c in 'A'..='Z' {
            if i == num {
                letter = c;
            }
            i += 1;
        }
        letter
    }
    pub fn get_color(letter: char) -> Color {
        match letter {
            'A' => Color::Blue,
            'B' => Color::Cyan,
            'C' => Color::Green,
            'D' => Color::Yellow,
            'E' => Color::Red,
            'F' => Color::Magenta,
            'G' => Color::Blue,
            'H' => Color::Cyan,
            'I' => Color::Green,
            'J' => Color::Yellow,
            'K' => Color::Red,
            'L' => Color::Magenta,
            'M' => Color::Blue,
            'N' => Color::Cyan,
            'O' => Color::Green,
            'P' => Color::Yellow,
            'Q' => Color::Red,
            'R' => Color::Magenta,
            'S' => Color::Blue,
            'T' => Color::Cyan,
            'U' => Color::Green,
            'V' => Color::Yellow,
            'W' => Color::Red,
            'X' => Color::Magenta,
            'Y' => Color::Blue,
            'Z' => Color::Cyan,
            _ => Color::White,
        }
    }
}

#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn get_gen_letter(){
        let mut i = 1;
        for letter in 'A'..'Z' {
            let gen_letter =LetterService::get_gen_letter(i);
            assert_eq!(gen_letter, letter);
            i += 1;
        }
    }

}
