class Anagram {
  constructor(baseWord) {
    this.baseDict = this.wordDict(baseWord);
  }

  wordDict = (word) => {
    return [...word.toLowerCase()].sort().join('');
  }

  isAnagram = (word) => {
    return this.baseDict === this.wordDict(word);
  }
  matches(wordList, ...rest) {
    if(typeof(wordList) === 'string') {
      return [wordList, ...rest].filter(this.isAnagram);
    }
    return wordList.filter(this.isAnagram);
  }
}

export default Anagram;
