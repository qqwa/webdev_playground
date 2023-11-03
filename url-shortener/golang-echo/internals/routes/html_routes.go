package routes

import (
	"database/sql"
	"log"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/qqwa/url-shortener/internals/shortener"
)

func Index(c echo.Context) error {
	return c.Render(http.StatusOK, "index", nil)
}

func Shorten(c echo.Context) error {
	return c.Render(http.StatusOK, "shorten", nil)
}

func ShortenPost(c echo.Context, db *sql.DB) error {
	m := map[string]string{}

	long_url := c.FormValue("long_url")
	m["long_url"] = long_url
	m["host"] = c.Request().Host

	url, err := shortener.CreateShortUrl(db, long_url)
	if err != nil {
		m["error"] = err.Error()
	}
	m["short_url"] = url.Short_url

	return c.Render(http.StatusOK, "shorten_post", m)
}

func Url(c echo.Context, db *sql.DB) error {
	short_url := c.Param("url")
	url, err := shortener.GetLongUrl(db, short_url)
	if err != nil {
		log.Println(err.Error())
	}
	return c.Redirect(http.StatusTemporaryRedirect, url.Long_url)
}

func FeedPolling(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func FeedSSE(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func FeedWS(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}
