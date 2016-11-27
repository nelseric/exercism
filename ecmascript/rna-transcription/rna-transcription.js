class Transcriptor {
  toRna(dna) {
    const rnaMap = {
      G: 'C',
      C: 'G',
      T: 'A',
      A: 'U',
    };
    return [...dna].map(n => rnaMap[n]).join('');
  }
}

export default Transcriptor;
