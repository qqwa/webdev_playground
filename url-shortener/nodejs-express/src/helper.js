const db = require('./queries')

function generateUrl(length) {
    const chars = 'abcdefghijklmnopqrstuvwxyz';
    let result = '';
    for (let i = 0; i < length; i++) {
        result = result + chars.charAt(Math.random()*chars.length);
    }
    return result;
}

const shorten = async (long_url) => {
    try {
        for (const i of [3, 5, 7, 9]) {
            const short_url = generateUrl(i)
            const res = await db.createUrl(short_url, long_url);
            return {
                "short_url": short_url,
                "long_url": long_url
            };
        }
    } catch (error) {
        return { "error": error };
    }
}

module.exports = {
    generateUrl,
    shorten
}