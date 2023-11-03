package routes

import (
	"database/sql"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/qqwa/url-shortener/internals/shortener"
)

type ApiError struct {
	Message string `json:"message"`
}

func GetUrls(c echo.Context, db *sql.DB) error {
	urls, err := shortener.GetLongUrls(db)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	return c.JSON(http.StatusOK, urls)
}

func PostUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func GetUrl(c echo.Context, db *sql.DB) error {
	short_url := c.Param("url")
	url, err := shortener.GetLongUrl(db, short_url)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	return c.JSON(http.StatusOK, url)
}

func PatchUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func DeleteUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}
