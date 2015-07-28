class Anagram(base_word: String) {
    def matches(words: Seq[String]): Seq[String] = {
        words.filter {
            word: String =>
                word.toLowerCase.sorted == base_word.toLowerCase.sorted &&
                    base_word.toLowerCase != word.toLowerCase
        }
    }
}