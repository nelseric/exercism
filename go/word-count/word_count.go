package wc

import (
	"regexp"
	"strings"
)

func WordCount(text string) Histogram {
	hist := Histogram{}

	wordMatcher := regexp.MustCompile(`\w+`)

	words := wordMatcher.FindAll([]byte(strings.ToLower(text)), -1)

	for _, word := range words {
		hist[string(word)]++
	}

	return hist
}
