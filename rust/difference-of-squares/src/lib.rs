pub fn sum_of_squares(n: i64) -> i64 {
  let mut sum: i64 = 0;
  for x in 1..(n+1) {
    sum += x * x
  }
  sum
}
pub fn square_of_sum(n: i64) -> i64 {
  let mut sum: i64 = 0;
  for x in 1..(n+1) {
    sum += x;
  }
  sum * sum
}

pub fn difference(n: i64) -> i64 {
   square_of_sum(n) - sum_of_squares(n)
}