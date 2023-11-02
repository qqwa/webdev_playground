package shortener

import (
	"strings"
)

func IsUrl(url string) bool {
	return strings.HasPrefix(url, "http://") ||
		strings.HasPrefix(url, "https://")
}
