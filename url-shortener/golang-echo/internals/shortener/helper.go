package shortener

import (
	"database/sql"
	"errors"
	"math/rand"
	"strings"
)

func IsUrl(url string) bool {
	return strings.HasPrefix(url, "http://") ||
		strings.HasPrefix(url, "https://")
}

var letters = []rune("abcdefghijklmnopqrstuvwxyz")

func genShortUrl(size int) string {
	bytes := make([]rune, size)
	for i := range bytes {
		bytes[i] = letters[rand.Intn(len(letters))]
	}
	return string(bytes)
}

func CreateShortUrl(db *sql.DB, long_url string) (string, error) {
	if IsUrl(long_url) {
		for _, i := range []int{5, 7, 9} {
			short_url := genShortUrl(i)
			_, err := db.Exec("INSERT INTO urls (short_url, long_url) VALUES ($1, $2);", short_url, long_url)
			if err != nil {
				// generated url existed alreay try again
			} else {
				return short_url, nil
			}
		}
		return "", errors.New("failed to generate url for " + long_url)
	} else {
		return "", errors.New(long_url + " is not an URL")
	}
}

type UrlDb struct {
	short_url string
	long_url  string
}

func GetLongUrl(db *sql.DB, short_url string) (string, error) {
	result := db.QueryRow("SELECT * FROM urls WHERE short_url = $1 LIMIT 1;", short_url)
	var urls UrlDb
	err := result.Scan(&urls.short_url, &urls.long_url)
	if err != nil {
		return "/", err
	} else {
		return urls.long_url, nil
	}
}
