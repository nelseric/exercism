class Bob {
  def hey(msg: String) = {
    if (is_not_empty(msg)) {
      if (is_shout(msg)) {
        "Woah, chill out!"
      } else if (is_question(msg)) {
        "Sure."
      } else {
        "Whatever."
      }
    } else {
      "Fine. Be that way!"
    }
  }

  def is_shout(msg: String) = {
    msg == msg.toUpperCase && has_upper_case(msg)
  }

  def has_upper_case(msg: String) = {
    // Is there a better way to check for regex partial matches?
    "[A-Z]+".r.findFirstIn(msg).isDefined
  }

  def is_question(msg: String) = {
    msg.endsWith("?")
  }
  def is_not_empty(msg: String) = {
    "\\S+".r.findFirstIn(msg).isDefined
  }
}