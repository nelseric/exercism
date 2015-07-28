package etl

import "utf8"

func Transform(input map[int][]string) map[string]int {
	dictionary := map[string]int{}

	for score, letters := range input {
		for _, letter := range letters {
			dictionary[strings.ToLower(letter)] = score
		}
	}

	return dictionary
}
