class Pangram {
  constructor(sentence) {
    this.sentence = sentence;
  }
  isPangram() {
    const charSet = new Set([...this.sentence.toLowerCase()]);
    const alphabet = [...charSet].filter(c => c.match(/[a-z]/));
    return alphabet.length === 26;
  }
}

export default Pangram;
