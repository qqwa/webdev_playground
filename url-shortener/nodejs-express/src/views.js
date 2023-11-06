const helper = require('./helper')
const db = require('./queries')

function index(req, res) {
    res.render('index.html');
}

const shorten = (request, response) => {
    response.render('shorten.html');
}

const shorten_post = async (request, response) => {
    data = await helper.shorten(request.body.long_url)
    data["host"] = request.hostname
    response.render('shorten_post.html', data);
}

const url = async (request, response) => {
    const short_url = request.params.url;
    try {
        const res = await db.getUrl(short_url);
        let body = res.rows[0];
        body.counter += 1;
        await db.incrementUrl(short_url);
        response.redirect(body.long_url);
    } catch (error) {
        response.redirect("/");
    }

}

module.exports = {
    index,
    shorten,
    shorten_post,
    url
};
