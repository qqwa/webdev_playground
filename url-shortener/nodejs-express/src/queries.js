require('dotenv').config();

const Pool = require('pg').Pool;
const connectionString = process.env.DATABASE_URL;
const pool = new Pool({connectionString});
