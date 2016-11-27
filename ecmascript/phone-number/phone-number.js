class PhoneNumber {
  constructor(number) {
    this._number = number.replace(/\D/g, '');
  }
  number() {
    return this._number;
  }
}

export default PhoneNumber;
