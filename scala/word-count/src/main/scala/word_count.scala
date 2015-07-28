class Phrase(phrase: String) {
    def wordCount() = {
        words.map {word => (word, countWord(word))}.toMap
    }

    def words() = {
        "[\\w']+".r.findAllIn(phrase.toLowerCase).toList
    }

    def countWord(word: String) = {
        words.count(_==word)
    }
}