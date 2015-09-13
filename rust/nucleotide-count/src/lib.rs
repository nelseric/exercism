use std::collections::HashMap;

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

  for ref c in dna.chars() {
    if let Some(count) = counts.get_mut(c){
      *count += 1;
    }
  }

  counts
}