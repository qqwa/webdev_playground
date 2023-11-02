package routes

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func GetLinks(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func PostLink(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func GetLink(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func PatchLink(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}

func DeleteLink(c echo.Context) error {
	return c.String(http.StatusOK, "TODO")
}
