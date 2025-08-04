use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let word_data = extract_word_data(&lower_word);
    let word_len = lower_word.chars().count();

    let mut set: HashSet<&str> = HashSet::new();

    for &w in possible_anagrams {
        if w.chars().count() != word_len {
            continue;
        }

        if w.to_lowercase() == lower_word {
            continue;
        }

        if compare_data(word_data.clone(), extract_word_data(&w.to_lowercase())) {
            set.insert(w);
        }
    }
    set
}

fn extract_word_data(word: &str) -> Vec<(char, i32)> {
    let mut word_data: Vec<(char, i32)> = Vec::new();
    for c_word in word.chars() {
        if let Some(index) = word_data.iter().position(|(c, _)| *c == c_word) {
            word_data[index].1 += 1;
        } else {
            word_data.push((c_word, 1));
        }
    }
    word_data
}

fn compare_data( data1: Vec<(char, i32)>, data2: Vec<(char, i32)> ) -> bool {
    data1.iter().all(|x| data2.contains(x)) && data2.iter().all(|x| data1.contains(x))
}
