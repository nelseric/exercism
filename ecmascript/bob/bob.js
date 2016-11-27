//
// This is only a SKELETON file for the 'Bob' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

class Bob {
  hey(message) {
    if (message.match(/^\s*$/)) {
      return 'Fine. Be that way!';
    }

    if (message.match(/[a-zA-Z]/) && message.toUpperCase() === message) {
      return 'Whoa, chill out!';
    }

    if (message.match(/\?$/)) {
      return 'Sure.';
    }

    return 'Whatever.';
  }
}

export default Bob;

