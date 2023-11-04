package routes

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/qqwa/url-shortener/internals/shortener"
	"github.com/textileio/go-threads/broadcast"
)

func Index(c echo.Context) error {
	return c.Render(http.StatusOK, "index", nil)
}

func Shorten(c echo.Context) error {
	return c.Render(http.StatusOK, "shorten", nil)
}

func ShortenPost(c echo.Context, db *sql.DB, b *broadcast.Broadcaster) error {
	m := map[string]string{}

	long_url := c.FormValue("long_url")
	m["long_url"] = long_url
	m["host"] = c.Request().Host

	url, err := shortener.CreateShortUrl(db, long_url)
	if err != nil {
		m["error"] = err.Error()
	}
	m["short_url"] = url.Short_url
	b.Send(shortener.UrlToEvent("created", *url))
	return c.Render(http.StatusOK, "shorten_post", m)
}

func Url(c echo.Context, db *sql.DB, b *broadcast.Broadcaster) error {
	short_url := c.Param("url")
	url, err := shortener.GetLongUrl(db, short_url)
	if err != nil {
		log.Println(err.Error())
	}
	shortener.IncrementShortUrl(db, short_url)
	url.Counter += 1
	b.Send(shortener.UrlToEvent("clicked", *url))
	return c.Redirect(http.StatusTemporaryRedirect, url.Long_url)
}

func FeedPolling(c echo.Context) error {
	return c.Render(http.StatusOK, "feed_poll", nil)
}

func FeedPollingData(c echo.Context, db *sql.DB) error {
	urls, err := shortener.GetLongUrls(db)
	if err != nil {
		return c.String(http.StatusOK, err.Error())
	}
	return c.Render(http.StatusOK, "urls", urls)
}

func FeedSSE(c echo.Context) error {
	return c.Render(http.StatusOK, "feed_sse", nil)
}

func SentSSEData(c echo.Context, listener *broadcast.Listener) error {
	c.Response().Header().Set(echo.HeaderContentType, "text/event-stream")
	c.Response().Header().Set(echo.HeaderCacheControl, "no-store")
	c.Response().Writer.Header()
	c.Response().Flush()

	for v := range listener.Channel() {
		data := fmt.Sprintf("%v", v)
		shortener.WriteServerSentEvent(c.Response(), "ping", data)
	}
	return nil
}

func FeedWS(c echo.Context) error {
	return c.Render(http.StatusOK, "feed_ws", nil)
}
