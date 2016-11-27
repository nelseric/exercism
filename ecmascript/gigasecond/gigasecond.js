class Gigasecond {
  constructor(start) {
    this.start = start;
  }

  date() {
    return new Date(this.start.getTime() + 1e12);
  }
}

export default Gigasecond;
