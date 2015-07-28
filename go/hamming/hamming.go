package hamming

import "unicode/utf8"

func Distance(strandA, strandB string) int {
	distance := 0

	lengthA := utf8.RuneCountInString(strandA)
	lengthB := utf8.RuneCountInString(strandB)

	if lengthA <= lengthB {
		for index := range strandA {
			if strandA[index] != strandB[index] {
				distance++
			}
		}
	} else {
		for index := range strandB {
			if strandA[index] != strandB[index] {
				distance++
			}
		}
	}

	return distance
}
