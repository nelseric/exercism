pub fn sorted(word: &String) -> String {
  // convert the String to an array of characters
  // collect is needed because it is an iterator?
  let mut sorted: Vec<char> = word.chars().collect();

  // sort the characters
  sorted.sort();

  // create an empty fixed length string that can hold all the sorted characters
  let mut output = String::with_capacity(sorted.len());

  for c in sorted {
    output.push(c);
  }

  output
}


pub fn lowercase(word: &str) -> String {
  let mut lower = String::with_capacity(word.len());
  for c in word.chars() {
    for low in c.to_lowercase() {
      lower.push(low);
    }
  }
  lower
}

// What does &[&'a str] mean?
// A reference to an array of string references who each have the lifetime a?
// where does lifetime a come from? in this instance,
//  am I binding the lifetime of this function call to the value 'a?
//  Or is it the lifetime of base?

pub fn anagrams_for<'a>(base: &'a str, inputs: &[&'a str]) -> Vec<&'a str> {
  let lower_base = lowercase(base);

  let sorted_base = sorted(&lower_base);

  let mut result = Vec::new();

  for input in inputs.iter() {
    let in_lower = lowercase(input);
    let in_sorted = sorted(&in_lower);
    if in_lower != lower_base && in_sorted == sorted_base {
      result.push(*input);
    }
  }

  result
}