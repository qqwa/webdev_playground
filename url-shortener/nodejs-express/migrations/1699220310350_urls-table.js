/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('urls', {
        short_url: { type: 'varchar', notNull: true },
        long_url: { type: 'varchar', notNull: true },
        counter: { type: 'integer', notNull: true, default: true },
    });

    pgm.createIndex('urls', 'short_url');
};

exports.down = pgm => {};
