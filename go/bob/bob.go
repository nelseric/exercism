package bob

import (
	"regexp"
	"strings"
)

func Hey(input string) string {
	if regexp.MustCompile(`\S+`).MatchString(input) {
		if isYelling(input) {
			return "Woah, chill out!"
		} else if isQuestion(input) {
			return "Sure."
		} else {
			return "Whatever."
		}
	} else {
		return "Fine. Be that way!"
	}
}

func isYelling(input string) bool {
	return strings.ToUpper(input) == input && regexp.MustCompile(`[A-Z]+`).MatchString(input)
}

func isQuestion(input string) bool {
	return regexp.MustCompile(`\?$`).MatchString(input)
}
