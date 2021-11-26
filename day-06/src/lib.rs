use std::fs;

#[derive(Debug)]
pub struct Group {
    text: String,
}

impl Group {
    pub fn get_question_count_answered_with_yes(&self) -> usize {
        let mut chars: Vec<char> = self.text.chars().collect();
        chars.sort();
        chars.dedup();
        chars.len()
    }
}

pub fn get_questions_answered_with_yes(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| group.get_question_count_answered_with_yes())
        .sum()
}

pub fn read_input() -> Vec<Group> {
    fs::read_to_string("inputs/day06.txt")
        .expect("Unable to read day 6 input.")
        .split("\n\n")
        .map(|text| text.replace("\n", ""))
        .map(|text| Group { text })
        .collect()
}
