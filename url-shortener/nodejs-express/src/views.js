const helper = require('./helper')

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

module.exports = {
    index,
    shorten,
    shorten_post
};
