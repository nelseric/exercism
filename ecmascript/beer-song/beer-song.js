class Beer {
  static verse(verseNum) {
    if (verseNum === 0) {
      return `No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
`;
    }
    if (verseNum === 1) {
      return `1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
`;
    }
    if (verseNum === 2) {
      return `2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.
`;
    }
    return `${verseNum} bottles of beer on the wall, ${verseNum} bottles of beer.
Take one down and pass it around, ${verseNum - 1} bottles of beer on the wall.
`;
  }

  static sing(startVerse = 99, endVerse = 0) {
    let verse = startVerse;
    const output = [];
    while (verse >= endVerse) {
      output.push(Beer.verse(verse));
      verse -= 1;
    }
    return output.join('\n');
  }
}

export default Beer;
