-- If you provide several instructions, separate those with TWO blank lines.
--
-- This schema is compiled as part of the program, as such you need to rebuild
-- (`cargo build`) the project after any schema modification.

CREATE TABLE IF NOT EXISTS fantoir_config
(
    key VARCHAR(63) NOT NULL
        CONSTRAINT fantoir_config_pk
            PRIMARY KEY,
        CONSTRAINT fantoir_config_key_format
            CHECK ( key ~ '^[a-zA-Z][a-zA-Z0-9_]*$' ),
    value VARCHAR(255)
);


INSERT INTO fantoir_config
    (key, value)
VALUES
    ('fantoir_table', '/*table*/fantoir')
ON CONFLICT (key) DO UPDATE
    SET value = excluded.value;
