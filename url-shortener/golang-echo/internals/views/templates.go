package views

import (
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

	return t
}

func addTemplate(path string) *template.Template {
	template, err := template.ParseFiles("templates/base.html", path)
	if err != nil {
		panic(fmt.Errorf("%s", err))
	}
	return template
}
