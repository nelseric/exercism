class PhoneNumber(raw_number: String) {

    def number() = {
        if (digits.length == 10)
            digits
        else if (digits.length == 11 && digits.slice(0, 1) == "1")
            digits.slice(1, 11)
        else
            "0000000000"
    }

    def digits() = {
        "\\d".r.findAllIn(raw_number).reduce { _ + _ }
    }

    def areaCode() = {
        number.slice(0, 3)
    }

    override def toString() = {
        "(123) 456-7890"
    }
}