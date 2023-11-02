package main

import (
	"fmt"
	"io"
	"text/template"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/qqwa/url-shortener/internals/routes"
)

type Template struct {
	templates *template.Template
}

func (t *Template) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return t.templates.ExecuteTemplate(w, name, data)
}

func main() {
	e := echo.New()
	t := &Template{
		templates: template.Must(template.ParseGlob("templates/*.html")),
	}
	fmt.Printf("%+v\n", t.templates)
	e.Renderer = t
	// HTML routes
	e.GET("/", routes.Index)
	e.GET("/shorten", routes.Shorten)
	e.POST("/shorten", routes.ShortenPost)
	e.GET("/l/:link", routes.Link)
	e.GET("/feed/polling", routes.FeedPolling)
	e.GET("/feed/sse", routes.FeedSSE)
	e.GET("/feed/ws", routes.FeedWS)

	// api routes
	e.GET("/api/links", routes.GetLinks)
	e.POST("/api/links", routes.PostLink)
	e.GET("/api/links/:link", routes.GetLink)
	e.PATCH("/api/links/:link", routes.PatchLink)
	e.DELETE("/api/links/:link", routes.DeleteLink)

	e.Use(middleware.Logger())
	e.Logger.Fatal(e.Start(":4000"))
}
