# Url Shortener

Example Project to test different web frameworks/approaches

## Browser/HTML Endpoints

+ `/` index page only showing some text
+ `/shorten` form to create new urls
+ `/l/:url` redirect to `/` if it doesn't exist, or redirect to shortend url
+ feed shows all urls with some stats and automatically includes new data
    + `/feed/polling`
    + `/feed/sse`
    + `/feed/ws`

## API/json Endpoints

+ GET `/api/urls` lists all shortend urls with some metadata
+ POST `/api/urls` create a new shortend url
+ GET `/api/urls/:url` lists shortend url with some metadata
+ PATCH `/api/urls/:url` change shortend url, but not metadata
+ DELETE `/api/urls/:url` deletes shortend url
