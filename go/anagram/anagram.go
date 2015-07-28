package anagram

import (
	"sort"
	"strings"
)

func Detect(_subject string, candidates []string) []string {
	matches := []string{}
	subject := strings.ToLower(_subject)
	subby := sortChars(subject)

	for _, _candidate := range candidates {
		candidate := strings.ToLower(_candidate)
		candy := sortChars(candidate)
		if candy == subby && candidate != subject {
			matches = append(matches, candidate)
		}
	}

	return matches
}

func sortChars(subject string) string {
	splitSubject := strings.Split(subject, "")
	sort.Strings(splitSubject)
	return strings.Join(splitSubject, "")
}
