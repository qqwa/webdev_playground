const db = require('./queries');
const helper = require('./helper')

async function getUrls(request, response) {
    const urls = await db.getUrls();
    response.status(200).json(urls.rows);
}

async function postUrl(request, response) {
    const { url } = request.body;
    try {
        for (const i of [3, 5, 7, 9]) {
            const short_url = helper.generateUrl(i)
            const res = await db.createUrl(short_url, url);
            response.status(200).json({"status": "ok", body: {
                "short_url": short_url,
                "long_url": url
            }});
            return;
        }
    } catch (error) {
        response.status(200).json({"status": "error", "message": error.detail});
    }
}

async function getUrl(request, response) {
    const url = request.params.url
    try {
        const res = await db.getUrl(url);
        let body = res.rows[0];
        body.counter += 1;
        await db.incrementUrl(url);
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