-- This table matches Wikidata entities and FANTOIR codes.
--
-- If you provide several instructions, separate those with TWO blank lines.
-- Indexes have to match every WHERE clause used against the database.
--
-- This schema is compiled as part of the program, as such you need to rebuild
-- (`cargo build`) the project after any schema modification.

CREATE TABLE IF NOT EXISTS /*table*/fantoir_wikidata
(
    -- Identifiers
    code_fantoir          char(11)    NOT NULL
        constraint /*index*/index_fantoir_wikidata_pk
            primary key,
    code_fantoir_wikidata char(11)    NOT NULL,

    -- Wikidata information
    item                  varchar(12) NOT NULL,
    item_label            text,
    what                  varchar(12) NOT NULL,

    -- Constraints
    UNIQUE (code_fantoir_wikidata)
);


CREATE INDEX CONCURRENTLY /*index*/index_fantoir_wikidata_voie_trigram
    ON /*table*/fantoir_wikidata
        USING gin (item_label gin_trgm_ops);
