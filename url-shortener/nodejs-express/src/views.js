const helper = require('./helper')
const db = require('./queries')

const EventEmitter = require('events').EventEmitter;
const sse_events = new EventEmitter;

function index(req, res) {
    res.render('index.html');
}

const shorten = (request, response) => {
    response.render('shorten.html');
}

const shorten_post = async (request, response) => {
    data = await helper.shorten(request.body.long_url)
    data["host"] = request.hostname
    sse_events.emit('event', {
        "what": "created",
        "long_url": data.long_url,
        "short_url": data.short_url
    });
    response.render('shorten_post.html', data);
}

const url = async (request, response) => {
    const short_url = request.params.url;
    try {
        const res = await db.getUrl(short_url);
        let body = res.rows[0];
        body.counter += 1;
        await db.incrementUrl(short_url);
        sse_events.emit('event', {
            "what": "clicked",
            "long_url": body.long_url,
            "short_url": body.short_url
        });
        response.redirect(body.long_url);
    } catch (error) {
        response.redirect("/");
    }
}

const feed_poll = (request, response) => {
    response.render('feed_poll.html');
}

const feed_sse = (request, response) => {
    response.render('feed_sse.html');
}

const sse = (request, response) => {
    const headers = {
        'Content-Type': 'text/event-stream',
        'Connection': 'keep-alive',
        'Cache-Control': 'no-cache'
    };
    response.writeHead(200, headers);

    sse_events.on('event', (event) => {
        const data = `data: <li><span class="font-bold">${event.what}</span> <a class="text-blue-700" href="/l/${event.short_url}">${request.hostname}/l/${event.short_url}</a> to <a class="text-blue-700" href="${event.long_url}">${event.long_url}</a></li>\n\n`;
        response.write(data);
    })
}

const feed_ws = (request, response) => {
    response.render('feed_ws.html');
}

module.exports = {
    index,
    shorten,
    shorten_post,
    url,
    feed_poll,
    feed_sse,
    sse,
    sse_events,
    feed_ws
};
