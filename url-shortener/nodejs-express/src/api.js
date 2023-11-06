const db = require('./queries');
const helper = require('./helper')
const sse_events = require('./views').sse_events;

async function getUrls(request, response) {
    const urls = await db.getUrls();
    response.status(200).json(urls.rows);
}

async function postUrl(request, response) {
    const { url } = request.body;
    data = await helper.shorten(url)
    sse_events.emit('event', {
        "what": "created",
        "long_url": url,
        "short_url": data.short_url
    });
    response.status(200).json({"status": "ok", body: {
        "short_url": data.short_url,
        "long_url": data.url
    }});
}

async function getUrl(request, response) {
    const url = request.params.url
    try {
        const res = await db.getUrl(url);
        let body = res.rows[0];
        body.counter += 1;
        await db.incrementUrl(url);
        sse_events.emit('event', {
            "what": "clicked",
            "long_url": body.long_url,
            "short_url": body.short_url
        });
        response.status(200).json({"status": "ok", body});
    } catch (error) {
        response.status(200).json({"status": "error", "message": error});
    }
}

async function patchUrl(request, response) {
    const short_url = request.params.url
    const { url } = request.body;
    try {
        const res = await db.update_url(short_url, url)
        let body = res.rows[0];
        response.status(200).json({"status": "ok", body});
    } catch (error) {
        response.status(200).json({"status": "error", "message": error});
    }
}

async function deleteUrl(request, response) {
    const url = request.params.url
    try {
        const res = await db.delete_url(url)
        response.status(200).json({"status": "ok"});
    } catch (error) {
        response.status(200).json({"status": "error", "message": error});
    }
}


module.exports = {
    getUrls,
    postUrl,
    getUrl,
    patchUrl,
    deleteUrl
};