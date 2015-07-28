class Bob
  hey: (string) =>
    if this.isSilience(string)
      "Fine. Be that way!"
    else if this.isShout(string)
      "Woah, chill out!"
    else if this.isQuestion(string)
      "Sure."
    else
      "Whatever."

  isQuestion: (string) =>
    /\?$/.test(string)
  isShout: (string) =>
    string == string.toUpperCase()
  isSilience: (string) =>
    ! /\S+/.test(string)
  

module.exports = Bob