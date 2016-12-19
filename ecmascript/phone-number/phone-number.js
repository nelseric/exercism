class PhoneNumber {
  constructor(number) {
    const clean = number.replace(/\D/g, '');
    const validated = clean.match(/^(?:1)?(\d{10})$/);
    this.match = validated ? validated[1] : '0000000000';
  }
  number() {
    return this.match;
  }
  areaCode() {
    return this.match.substr(0, 3);
  }
  toString() {
    const exchg = this.match.substr(3, 3);
    const num = this.match.substr(6, 4);
    return `(${this.areaCode()}) ${exchg}-${num}`;
  }

}

export default PhoneNumber;
