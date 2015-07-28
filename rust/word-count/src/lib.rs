use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn lowercase(word: &str) -> String {
  let mut lower = String::with_capacity(word.len());
  for c in word.chars() {
    for low in c.to_lowercase() {
      lower.push(low);
    }
  }
  lower
}


pub fn word_count(words: &str) -> HashMap<String, u32> {
  
  let mut result: HashMap<String, u32> = HashMap::new();

  let lower = lowercase(words);

  for word in lower.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
    match result.entry(word.to_string()) {
      Entry::Vacant(entry) => { entry.insert(1); },
      Entry::Occupied(mut entry) => { *entry.get_mut() += 1;}
    }
  }

  result
}