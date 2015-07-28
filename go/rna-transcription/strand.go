package strand

import "strings"

func ToRna(rna string) string {
	transcribed := []string{}

	nucleotide_map := map[rune]string{
		'G': "C",
		'C': "G",
		'A': "U",
		'T': "A",
	}

	for _, nucleotide := range rna {
		transcribed = append(transcribed, nucleotide_map[nucleotide])
	}

	return strings.Join(transcribed, "")
}
