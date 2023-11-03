CREATE TABLE IF NOT EXISTS urls (
    short_url VARCHAR NOT NULL UNIQUE,
    long_url VARCHAR NOT NULL
);

CREATE INDEX short_urls ON urls (short_url);
