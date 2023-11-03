package main

import (
	"database/sql"
	"log"
	"os"

	_ "github.com/lib/pq"

	"github.com/joho/godotenv"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/qqwa/url-shortener/internals/routes"
	"github.com/qqwa/url-shortener/internals/views"
)

func main() {
	godotenv.Load()
	db_url := os.Getenv("DATABASE_URL")

	db, err := sql.Open("postgres", db_url)
	if err != nil {
		log.Fatal(err)
	}

	e := echo.New()
	e.Renderer = views.GetTemplates()
	// HTML routes
	e.GET("/", routes.Index)
	e.GET("/shorten", routes.Shorten)
	e.POST("/shorten", func(c echo.Context) error {
		return routes.ShortenPost(c, db)
	})
	e.GET("/l/:url", func(c echo.Context) error {
		return routes.Url(c, db)
	})
	e.GET("/feed/polling", routes.FeedPolling)
	e.GET("/feed/sse", routes.FeedSSE)
	e.GET("/feed/ws", routes.FeedWS)

	// api routes
	e.GET("/api/urls", routes.GetUrls)
	e.POST("/api/urls", routes.PostUrl)
	e.GET("/api/urls/:url", routes.GetUrl)
	e.PATCH("/api/urls/:url", routes.PatchUrl)
	e.DELETE("/api/urls/:url", routes.DeleteUrl)

	e.Use(middleware.Logger())
	e.Logger.Fatal(e.Start(":4000"))
}
