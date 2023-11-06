const express = require('express');
const nunjucks = require('nunjucks');
const bodyParser = require('body-parser');

const db = require('./queries')
const views = require('./views')
const api = require('./api')

const app = express();
const port = 4000;

nunjucks.configure('views', {
    autoescape: true,
    express: app
});

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

app.use('/assets', express.static('assets'));

// html/browser
app.get('/', views.index);
app.get('/shorten', views.shorten);
app.post('/shorten', views.shorten_post);
app.get('/l/:url', views.url);
app.get('/feed/polling', views.feed_poll);
app.get('/feed/sse', views.feed_sse);
app.get('/sse', views.sse);

// api/json
app.get('/api/urls', api.getUrls);
app.post('/api/urls', api.postUrl);
app.get('/api/urls/:url', api.getUrl)
app.patch('/api/urls/:url', api.patchUrl)
app.delete('/api/urls/:url', api.deleteUrl)

app.listen(port, () => {
    console.log(`Started server on port ${port}`);
});
