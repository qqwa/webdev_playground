package main

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/qqwa/url-shortener/internals/routes"
	"github.com/qqwa/url-shortener/internals/views"
)

func main() {
	e := echo.New()
	e.Renderer = views.GetTemplates()
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
