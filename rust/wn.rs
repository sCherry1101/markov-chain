use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

type Word = Arc<str>;
type ChainKey = (Word, Word);

pub struct Markov {
    chain: HashMap<ChainKey, Vec<Word>>,
    start_words: Vec<ChainKey>,
}

impl Markov {
    pub fn new() -> Self {
        Self {
            chain: HashMap::new(),
            start_words: Vec::new(),
        }
    }

    fn clean_text(text: &str) -> String {
        text.replace('\n', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn train(&mut self, raw_text: &str) {
        let cleaned = Self::clean_text(raw_text);
        let words: Vec<Word> = cleaned.split_whitespace().map(Into::into).collect();

        if words.len() < 3 {
            return;
        }

        for i in 0..words.len() - 2 {
            let w1 = Arc::clone(&words[i]);
            let w2 = Arc::clone(&words[i + 1]);
            let w3 = Arc::clone(&words[i + 2]);

            let key = (w1, w2);

            if key.0.ends_with('.') && key.1.chars().next().map_or(false, |c| c.is_uppercase()) {
                self.start_words.push((Arc::clone(&key.1), Arc::clone(&w3)));
            }

            self.chain.entry(key).or_insert_with(Vec::new).push(w3);
        }
    }

    pub fn generate(&self, length: usize) -> String {
        let mut rng = rand::rng();

        let mut current_key = if !self.start_words.is_empty() && rng.random_bool(0.8) {
            self.start_words.choose(&mut rng).unwrap().clone()
        } else {
            let keys: Vec<&ChainKey> = self.chain.keys().collect();
            (*keys.choose(&mut rng).expect("Chain is empty")).clone()
        };

        let mut result = vec![current_key.0.to_string(), current_key.1.to_string()];

        for _ in 0..length {
            if let Some(choices) = self.chain.get(&current_key) {
                let next_word = choices.choose(&mut rng).unwrap();
                result.push(next_word.to_string());

                if next_word.ends_with('.') {
                    break;
                }

                current_key = (Arc::clone(&current_key.1), Arc::clone(next_word));
            } else {
                break;
            }
        }

        result.join(" ")
    }
}

fn main() {
    let content = fs::read_to_string("wn.txt").expect("Could not read file");
    let mut ai = Markov::new();
    ai.train(&content);
    println!("{}", ai.generate(80));
}
