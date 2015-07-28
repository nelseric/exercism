package accumulate

func Accumulate(list []string, converter func(string) string) []string {
	temp := []string{}

	for _, item := range list {
		temp = append(temp, converter(item))
	}

	return temp
}
