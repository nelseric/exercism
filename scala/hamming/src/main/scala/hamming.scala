object Hamming {
  def compute(str_a: String, str_b: String) = {
    str_a.zip(str_b).map {a: (Char, Char) =>  if(a._1 != a._2) 1 else 0 }.sum
  }
}