require('dotenv').config();

const Pool = require('pg').Pool;
const connectionString = process.env.DATABASE_URL;
const pool = new Pool({connectionString});

const getUrls = async () => {
    const res = await pool.query('SELECT * FROM urls');
    return res;
};

const createUrl = async (short_url, long_url) => {
    const res = await pool.query('INSERT INTO urls (short_url, long_url) VALUES ($1, $2)', [short_url, long_url]);
    return res;
}

const getUrl = async (short_url) => {
    const res = await pool.query('SELECT * from urls WHERE short_url = $1', [short_url]);
    return res;
}

const incrementUrl = async(short_url) => {
    const res = await pool.query('UPDATE urls SET counter = counter + 1 WHERE short_url = $1', [short_url]);
    return res;
}

const update_url = async(short_url, long_url) => {
    const res = await pool.query('UPDATE urls SET long_url = $1 WHERE short_url = $2 RETURNING *', [long_url, short_url]);
    return res;
}

const delete_url = async(short_url) => {
    const res = await pool.query('DELETE FROM urls WHERE short_url = $1', [short_url]);
    return res;
}

module.exports = {
    getUrls,
    getUrl,
    createUrl,
    update_url,
    delete_url,
    incrementUrl
}
