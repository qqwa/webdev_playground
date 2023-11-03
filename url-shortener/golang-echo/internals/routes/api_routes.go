package routes

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func GetUrls(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func PostUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func GetUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func PatchUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func DeleteUrl(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}
