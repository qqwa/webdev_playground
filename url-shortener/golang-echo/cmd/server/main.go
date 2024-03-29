package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	_ "github.com/lib/pq"
	"github.com/textileio/go-threads/broadcast"
	"golang.org/x/net/websocket"

	"github.com/joho/godotenv"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/qqwa/url-shortener/internals/routes"
	"github.com/qqwa/url-shortener/internals/views"
)

func ws_handler(c echo.Context, listener *broadcast.Listener) error {
	websocket.Handler(func(ws *websocket.Conn) {
		defer ws.Close()
		for v := range listener.Channel() {
			data := fmt.Sprintf("<div hx-swap-oob=\"afterbegin:#event\">%v</div>", v)
			err := websocket.Message.Send(ws, data)
			if err != nil {
				c.Logger().Error(err)
				break
			}
		}
	}).ServeHTTP(c.Response(), c.Request())
	return nil
}

func main() {
	godotenv.Load()
	db_url := os.Getenv("DATABASE_URL")

	db, err := sql.Open("postgres", db_url)
	if err != nil {
		log.Fatal(err)
	}

	b := broadcast.NewBroadcaster(10)

	e := echo.New()
	e.Renderer = views.GetTemplates()
	// HTML routes
	e.GET("/", routes.Index)
	e.GET("/shorten", routes.Shorten)
	e.POST("/shorten", func(c echo.Context) error {
		return routes.ShortenPost(c, db, b)
	})
	e.GET("/l/:url", func(c echo.Context) error {
		return routes.Url(c, db, b)
	})
	e.GET("/feed/polling", routes.FeedPolling)
	e.GET("/feed/polling/data", func(c echo.Context) error {
		return routes.FeedPollingData(c, db)
	})
	e.GET("/feed/sse", routes.FeedSSE)
	e.GET("/sse", func(c echo.Context) error {
		return routes.SentSSEData(c, b.Listen())
	})
	e.GET("/feed/ws", routes.FeedWS)
	e.GET("/ws", func(c echo.Context) error {
		return ws_handler(c, b.Listen())
	})

	// api routes
	e.GET("/api/urls", func(c echo.Context) error {
		return routes.GetUrls(c, db)
	})
	e.POST("/api/urls", func(c echo.Context) error {
		return routes.PostUrl(c, db, b)
	})
	e.GET("/api/urls/:url", func(c echo.Context) error {
		return routes.GetUrl(c, db, b)
	})
	e.PATCH("/api/urls/:url", func(c echo.Context) error {
		return routes.PatchUrl(c, db)
	})
	e.DELETE("/api/urls/:url", func(c echo.Context) error {
		return routes.DeleteUrl(c, db)
	})

	e.Use(middleware.Logger())
	e.Logger.Fatal(e.Start(":4000"))
}
