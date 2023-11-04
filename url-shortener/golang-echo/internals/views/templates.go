package views

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"text/template"

	"github.com/labstack/echo/v4"
)

type Template struct {
	templates map[string]*template.Template
}

func (t Template) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return t.templates[name].Execute(w, data)
}

func GetTemplates() Template {
	t := Template{
		templates: make(map[string]*template.Template),
	}

	t.templates["index"] = addTemplate("templates/index.html")
	t.templates["shorten"] = addTemplate("templates/shorten.html")
	t.templates["shorten_post"] = addTemplate("templates/shorten_post.html")
	t.templates["feed_poll"] = addTemplate("templates/feed_poll.html")
	t.templates["urls"] = addTemplateStandalone("templates/urls.html")
	t.templates["feed_sse"] = addTemplate("templates/feed_sse.html")

	return t
}

func addTemplate(path string) *template.Template {
	template, err := template.ParseFiles("templates/base.html", path)
	if err != nil {
		panic(fmt.Errorf("%s", err))
	}
	return template
}

func addTemplateStandalone(path string) *template.Template {
	template, err := template.ParseFiles(path)
	if err != nil {
		panic(fmt.Errorf("%s", err))
	}
	return template
}

func WriteServerSentEvent(response *echo.Response, event string, data any) {
	m := map[string]any{
		"data": data,
	}
	buff := bytes.NewBuffer([]byte{})
	encoder := json.NewEncoder(buff)
	err := encoder.Encode(m)
	if err != nil {
		return
	}

	response.Writer.Write([]byte(fmt.Sprintf("event: %s\n", event)))
	response.Writer.Write([]byte(fmt.Sprintf("data: %v\n\n", buff.String())))
	response.Flush()
}
