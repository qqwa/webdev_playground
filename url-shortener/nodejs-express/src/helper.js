
function generateUrl(length) {
    const chars = 'abcdefghijklmnopqrstuvwxyz';
    let result = '';
    for (let i = 0; i < length; i++) {
        result = result + chars.charAt(Math.random()*chars.length);
    }
    return result;
}

module.exports = {
    generateUrl
}