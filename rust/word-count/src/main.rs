extern crate word_count;

fn main() {
  let words = "car : carpet as java : javascript!!&@$%^&";
  println!("{:?}", word_count::word_count(words));
}