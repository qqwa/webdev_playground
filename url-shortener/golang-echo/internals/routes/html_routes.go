package routes

import (
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

func ShortenPost(c echo.Context) error {
	m := map[string]string{}

	long_url := c.FormValue("long_url")
	m["long_url"] = long_url

	if shortener.IsUrl(long_url) {
		// TODO: create and save to database
		m["short_url"] = "TODO"
	} else {
		m["error"] = long_url + " is not an URL"
	}

	return c.Render(http.StatusOK, "shorten_post", m)
}

func Link(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
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
