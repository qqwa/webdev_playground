package routes

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func Index(c echo.Context) error {
	return c.Render(http.StatusOK, "index.html", map[string]string{})
}

func Shorten(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func ShortenPost(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
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
