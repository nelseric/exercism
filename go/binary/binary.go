package binary

func ToDecimal(binary string) int {
	value := 0
	for _, digit := range binary {
		if digit == '0' {
			value = (value << 1)
		} else if digit == '1' {
			value = (value << 1) + 1
		} else {
			return 0
		}
	}
	return value
}
