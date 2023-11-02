# Url Shortener

Example Project to test different web frameworks/approaches

## Browser/HTML Endpoints

+ `/` index page only showing some text
+ `/shorten` form to create new links
+ `/l/:link` redirect to `/` if it doesn't exist, or redirect to shortend link
+ feed shows all links with some stats and automatically includes new data
    + `/feed/polling`
    + `/feed/sse`
    + `/feed/ws`

## API/json Endpoints

+ GET `/api/links` lists all shortend links with some metadata
+ POST `/api/links` create a new shortend link
+ GET `/api/links/:link` lists shortend link with some metadata
+ PATCH `/api/links/:link` change shortend link, but not metadata
+ DELETE `/api/links/:link` deletes shortend link
