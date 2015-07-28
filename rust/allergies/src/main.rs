extern crate allergies;

use allergies::*;

fn main() {
  let a = Allergies(129);
  
  for ag in Allergies::allergens().into_iter() {
    println!("{:?}: {}", ag, a.is_allergic_to(&ag));
  }
}