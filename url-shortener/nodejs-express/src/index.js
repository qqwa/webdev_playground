const express = require('express');
const nunjucks = require('nunjucks');
const bodyParser = require('body-parser');

const db = require('./queries')
const views = require('./views')

const app = express();
const port = 4000;

nunjucks.configure('views', {
    autoescape: true,
    express: app
});

app.get('/', views.index);

app.listen(port, () => {
    console.log(`Started server on port ${port}`);
});
