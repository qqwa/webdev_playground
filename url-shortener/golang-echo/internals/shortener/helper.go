package shortener

import (
	"database/sql"
	"errors"
	"log"
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

func CreateShortUrl(db *sql.DB, long_url string) (*UrlDb, error) {
	if IsUrl(long_url) {
		for _, i := range []int{5, 7, 9} {
			short_url := genShortUrl(i)
			_, err := db.Exec("INSERT INTO urls (short_url, long_url) VALUES ($1, $2);", short_url, long_url)
			if err != nil {
				// generated url existed alreay try again
			} else {
				return &UrlDb{Short_url: short_url, Long_url: long_url}, nil
			}
		}
		return nil, errors.New("failed to generate url for " + long_url)
	} else {
		return nil, errors.New(long_url + " is not an URL")
	}
}

type UrlDb struct {
	Short_url string `json:"short_url"`
	Long_url  string `json:"long_url"`
}

func GetLongUrl(db *sql.DB, short_url string) (*UrlDb, error) {
	result := db.QueryRow("SELECT * FROM urls WHERE short_url = $1 LIMIT 1;", short_url)
	var url UrlDb
	err := result.Scan(&url.Short_url, &url.Long_url)
	if err != nil {
		return nil, err
	} else {
		return &url, nil
	}
}

func GetLongUrls(db *sql.DB) ([]UrlDb, error) {
	rows, err := db.Query(("SELECT * FROM urls;"))
	if err != nil {
		log.Println("error: " + err.Error())
	}
	defer rows.Close()
	urls := make([]UrlDb, 1)
	for rows.Next() {
		var url UrlDb
		if err := rows.Scan(&url.Short_url, &url.Long_url); err != nil {
			log.Println("error: " + err.Error())
		}
		urls = append(urls, url)
	}
	return urls, nil
}
