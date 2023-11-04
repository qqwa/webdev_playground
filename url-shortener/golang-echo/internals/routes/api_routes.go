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

type LongUrl struct {
	Url string `json:"url"`
}

func PostUrl(c echo.Context, db *sql.DB) error {
	var long_url LongUrl
	err := c.Bind(&long_url)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	url, err := shortener.CreateShortUrl(db, long_url.Url)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	url.Short_url = shortener.ShortUrlToFullUrl(c.Request().Host, url.Short_url)
	return c.JSON(http.StatusOK, url)
}

func GetUrl(c echo.Context, db *sql.DB) error {
	short_url := c.Param("url")
	url, err := shortener.GetLongUrl(db, short_url)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	shortener.IncrementShortUrl(db, short_url)
	return c.JSON(http.StatusOK, url)
}

func PatchUrl(c echo.Context, db *sql.DB) error {
	var long_url LongUrl
	err := c.Bind(&long_url)
	short_url := c.Param("url")
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	url, err := shortener.UpdateShortUrl(db, short_url, long_url.Url)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	url.Short_url = shortener.ShortUrlToFullUrl(c.Request().Host, url.Short_url)
	return c.JSON(http.StatusOK, url)
}

func DeleteUrl(c echo.Context, db *sql.DB) error {
	short_url := c.Param("url")
	err := shortener.DeleteUrl(db, short_url)
	if err != nil {
		return c.JSON(http.StatusOK, ApiError{Message: err.Error()})
	}
	return c.JSON(http.StatusOK, map[string]string{
		"message": "deleted",
	})
}
