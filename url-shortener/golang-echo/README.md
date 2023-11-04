# golang echo

Exploring backend development with go using echo and text/template.

Htmx is used for the interactive pages `/feed/*`

## Migrations

For migrations the golang-migrate package is used. Install using:  
`go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest`

migrate -database ${DATABASE_URL} -path db/migrations up