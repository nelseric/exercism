// The example is very concise

pub fn hamming_distance(a: &str, b: &str) -> usize {
  a.chars().zip(
      b.chars()
    ).filter(
      |&(a,b)| a != b
    ).count()
}