use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn count(nucleotide: char, dna: &str) -> usize {
  let counts = nucleotide_counts(dna);
  *counts.get(&nucleotide).unwrap()
}

pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
  let mut counts = HashMap::new();

  counts.insert('G', 0);
  counts.insert('C', 0);
  counts.insert('A', 0);
  counts.insert('T', 0);

  for c in dna.chars() {
    let mut count = counts.entry(c).or_insert(0);
    *count += 1;
  }

  counts
}