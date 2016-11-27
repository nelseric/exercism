class Words {
  count(words) {
    const tokens = words.trim()
                        .split(/\s+/)
                        .map(word => word.toLowerCase());

    const dict = new Set(tokens);
    const counts = {};

    for (const word of dict) {
      counts[word] = tokens.filter(token => token === word).length;
    }
    return counts;
  }
}

export default Words;
